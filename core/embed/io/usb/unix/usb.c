/*
 * This file is part of the Trezor project, https://trezor.io/
 *
 * Copyright (c) SatoshiLabs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <trezor_rtl.h>

#include <arpa/inet.h>
#include <fcntl.h>
#include <stdlib.h>
#include <sys/poll.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

#include <io/usb.h>
#include "profile.h"

#include "memzero.h"

// emulator opens UDP server and emulates HID/WebUSB interfaces
// gracefully ignores all other USB interfaces

#define USBD_MAX_NUM_INTERFACES 8

typedef enum {
  USB_IFACE_TYPE_DISABLED = 0,
  USB_IFACE_TYPE_VCP = 1,
  USB_IFACE_TYPE_HID = 2,
  USB_IFACE_TYPE_WEBUSB = 3,
} usb_iface_type_t;

static struct {
  usb_iface_type_t type;
  uint16_t port;
  int sock;
  struct sockaddr_in si_me, si_other;
  socklen_t slen;
  uint8_t msg[64];
  int msg_len;
} usb_ifaces[USBD_MAX_NUM_INTERFACES];

secbool usb_init(const usb_dev_info_t *dev_info) {
  (void)dev_info;
  for (int i = 0; i < USBD_MAX_NUM_INTERFACES; i++) {
    usb_ifaces[i].type = USB_IFACE_TYPE_DISABLED;
    usb_ifaces[i].port = 0;
    usb_ifaces[i].sock = -1;
    memzero(&usb_ifaces[i].si_me, sizeof(struct sockaddr_in));
    memzero(&usb_ifaces[i].si_other, sizeof(struct sockaddr_in));
    memzero(&usb_ifaces[i].msg, sizeof(usb_ifaces[i].msg));
    usb_ifaces[i].slen = 0;
    usb_ifaces[i].msg_len = 0;
  }
  return sectrue;
}

void usb_deinit(void) { usb_stop(); }

secbool usb_start(void) {
  const char *ip = getenv("TREZOR_UDP_IP");

  // iterate interfaces
  for (int i = 0; i < USBD_MAX_NUM_INTERFACES; i++) {
    // skip if not HID or WebUSB interface
    if (usb_ifaces[i].type != USB_IFACE_TYPE_HID &&
        usb_ifaces[i].type != USB_IFACE_TYPE_WEBUSB) {
      continue;
    }

    usb_ifaces[i].sock = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    ensure(sectrue * (usb_ifaces[i].sock >= 0), NULL);

    fcntl(usb_ifaces[i].sock, F_SETFL, O_NONBLOCK);

    usb_ifaces[i].si_me.sin_family = AF_INET;
    if (ip) {
      usb_ifaces[i].si_me.sin_addr.s_addr = inet_addr(ip);
    } else {
      usb_ifaces[i].si_me.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    }
    usb_ifaces[i].si_me.sin_port = htons(usb_ifaces[i].port);

    ensure(sectrue * (0 == bind(usb_ifaces[i].sock,
                                (struct sockaddr *)&usb_ifaces[i].si_me,
                                sizeof(struct sockaddr_in))),
           NULL);
  }

  return sectrue;
}

void usb_stop(void) {
  for (int i = 0; i < USBD_MAX_NUM_INTERFACES; i++) {
    if (usb_ifaces[i].sock >= 0) {
      close(usb_ifaces[i].sock);
      usb_ifaces[i].sock = -1;
    }
  }
}

secbool usb_hid_add(const usb_hid_info_t *info) {
  if (info->iface_num < USBD_MAX_NUM_INTERFACES &&
      usb_ifaces[info->iface_num].type == USB_IFACE_TYPE_DISABLED) {
    usb_ifaces[info->iface_num].type = USB_IFACE_TYPE_HID;
    usb_ifaces[info->iface_num].port = info->emu_port;
  }
  return sectrue;
}

secbool usb_webusb_add(const usb_webusb_info_t *info) {
  if (info->iface_num < USBD_MAX_NUM_INTERFACES &&
      usb_ifaces[info->iface_num].type == USB_IFACE_TYPE_DISABLED) {
    usb_ifaces[info->iface_num].type = USB_IFACE_TYPE_WEBUSB;
    usb_ifaces[info->iface_num].port = info->emu_port;
  }
  return sectrue;
}

secbool usb_vcp_add(const usb_vcp_info_t *info) {
  if (info->iface_num < USBD_MAX_NUM_INTERFACES &&
      usb_ifaces[info->iface_num].type == USB_IFACE_TYPE_DISABLED) {
    usb_ifaces[info->iface_num].type = USB_IFACE_TYPE_VCP;
    usb_ifaces[info->iface_num].port = info->emu_port;
  }
  return sectrue;
}

static secbool usb_emulated_poll_read(uint8_t iface_num) {
  if (usb_ifaces[iface_num].msg_len > 0) {
    return sectrue;
  }

  struct pollfd fds[] = {
      {usb_ifaces[iface_num].sock, POLLIN, 0},
  };
  int res = poll(fds, 1, 0);

  if (res <= 0) {
    return secfalse;
  }

  struct sockaddr_in si;
  socklen_t sl = sizeof(si);
  ssize_t r = recvfrom(usb_ifaces[iface_num].sock, usb_ifaces[iface_num].msg,
                       sizeof(usb_ifaces[iface_num].msg), MSG_DONTWAIT,
                       (struct sockaddr *)&si, &sl);
  if (r <= 0) {
    return secfalse;
  }

  usb_ifaces[iface_num].si_other = si;
  usb_ifaces[iface_num].slen = sl;
  static const char *ping_req = "PINGPING";
  static const char *ping_resp = "PONGPONG";
  if (r == strlen(ping_req) &&
      0 == memcmp(ping_req, usb_ifaces[iface_num].msg, strlen(ping_req))) {
    if (usb_ifaces[iface_num].slen > 0) {
      sendto(usb_ifaces[iface_num].sock, ping_resp, strlen(ping_resp),
             MSG_DONTWAIT,
             (const struct sockaddr *)&usb_ifaces[iface_num].si_other,
             usb_ifaces[iface_num].slen);
    }
    memzero(usb_ifaces[iface_num].msg, sizeof(usb_ifaces[iface_num].msg));
    return secfalse;
  }

  usb_ifaces[iface_num].msg_len = r;

  return sectrue;
}

static secbool usb_emulated_poll_write(uint8_t iface_num) {
  struct pollfd fds[] = {
      {usb_ifaces[iface_num].sock, POLLOUT, 0},
  };
  int r = poll(fds, 1, 0);
  return sectrue * (r > 0);
}

static int usb_emulated_read(uint8_t iface_num, uint8_t *buf, uint32_t len) {
  if (usb_ifaces[iface_num].msg_len > 0) {
    if (usb_ifaces[iface_num].msg_len < len) {
      len = usb_ifaces[iface_num].msg_len;
    }
    memcpy(buf, usb_ifaces[iface_num].msg, len);
    usb_ifaces[iface_num].msg_len = 0;
    memzero(usb_ifaces[iface_num].msg, sizeof(usb_ifaces[iface_num].msg));
    return len;
  }

  return 0;
}

static int usb_emulated_write(uint8_t iface_num, const uint8_t *buf,
                              uint32_t len) {
  ssize_t r = len;
  if (usb_ifaces[iface_num].slen > 0) {
    r = sendto(usb_ifaces[iface_num].sock, buf, len, MSG_DONTWAIT,
               (const struct sockaddr *)&usb_ifaces[iface_num].si_other,
               usb_ifaces[iface_num].slen);
  }
  return r;
}

secbool usb_hid_can_read(uint8_t iface_num) {
  if (iface_num >= USBD_MAX_NUM_INTERFACES ||
      usb_ifaces[iface_num].type != USB_IFACE_TYPE_HID) {
    return secfalse;
  }
  return usb_emulated_poll_read(iface_num);
}

secbool usb_webusb_can_read(uint8_t iface_num) {
  if (iface_num >= USBD_MAX_NUM_INTERFACES ||
      usb_ifaces[iface_num].type != USB_IFACE_TYPE_WEBUSB) {
    return secfalse;
  }
  return usb_emulated_poll_read(iface_num);
}

secbool usb_hid_can_write(uint8_t iface_num) {
  if (iface_num >= USBD_MAX_NUM_INTERFACES ||
      usb_ifaces[iface_num].type != USB_IFACE_TYPE_HID) {
    return secfalse;
  }
  return usb_emulated_poll_write(iface_num);
}

secbool usb_webusb_can_write(uint8_t iface_num) {
  if (iface_num >= USBD_MAX_NUM_INTERFACES ||
      usb_ifaces[iface_num].type != USB_IFACE_TYPE_WEBUSB) {
    return secfalse;
  }
  return usb_emulated_poll_write(iface_num);
}

int usb_hid_read(uint8_t iface_num, uint8_t *buf, uint32_t len) {
  if (iface_num >= USBD_MAX_NUM_INTERFACES ||
      usb_ifaces[iface_num].type != USB_IFACE_TYPE_HID) {
    return 0;
  }
  return usb_emulated_read(iface_num, buf, len);
}

int usb_webusb_read(uint8_t iface_num, uint8_t *buf, uint32_t len) {
  if (iface_num >= USBD_MAX_NUM_INTERFACES ||
      usb_ifaces[iface_num].type != USB_IFACE_TYPE_WEBUSB) {
    return 0;
  }
  return usb_emulated_read(iface_num, buf, len);
}

int usb_webusb_read_blocking(uint8_t iface_num, uint8_t *buf, uint32_t len,
                             int timeout) {
  const uint32_t start = clock();
  while (sectrue != usb_webusb_can_read(iface_num)) {
    if (timeout >= 0 &&
        (1000 * (clock() - start)) / CLOCKS_PER_SEC >= timeout) {
      return 0;  // Timeout
    }
  }
  return usb_webusb_read(iface_num, buf, len);
}

int usb_hid_write(uint8_t iface_num, const uint8_t *buf, uint32_t len) {
  if (iface_num >= USBD_MAX_NUM_INTERFACES ||
      usb_ifaces[iface_num].type != USB_IFACE_TYPE_HID) {
    return 0;
  }
  return usb_emulated_write(iface_num, buf, len);
}

int usb_hid_write_blocking(uint8_t iface_num, const uint8_t *buf, uint32_t len,
                           int timeout) {
  const uint32_t start = clock();
  while (sectrue != usb_hid_can_write(iface_num)) {
    if (timeout >= 0 &&
        (1000 * (clock() - start)) / CLOCKS_PER_SEC >= timeout) {
      return 0;  // Timeout
    }
  }
  return usb_hid_write(iface_num, buf, len);
}

int usb_webusb_write(uint8_t iface_num, const uint8_t *buf, uint32_t len) {
  if (iface_num >= USBD_MAX_NUM_INTERFACES ||
      usb_ifaces[iface_num].type != USB_IFACE_TYPE_WEBUSB) {
    return 0;
  }
  return usb_emulated_write(iface_num, buf, len);
}

int usb_webusb_write_blocking(uint8_t iface_num, const uint8_t *buf,
                              uint32_t len, int timeout) {
  const uint32_t start = clock();
  while (sectrue != usb_webusb_can_write(iface_num)) {
    if (timeout >= 0 &&
        (1000 * (clock() - start)) / CLOCKS_PER_SEC >= timeout) {
      return 0;  // Timeout
    }
  }
  return usb_webusb_write(iface_num, buf, len);
}

void mp_hal_set_vcp_iface(int iface_num) {}

secbool usb_configured(void) {
  if (access(profile_usb_disconnect_path(), F_OK) == 0) {
    return secfalse;
  }

  return sectrue;
}

usb_event_t usb_get_event(void) { return USB_EVENT_NONE; }

void usb_get_state(usb_state_t *state) {
  state->configured = usb_configured() == sectrue;
}
