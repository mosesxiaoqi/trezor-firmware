#ifndef _TREZOR_T3W1_H
#define _TREZOR_T3W1_H

#define HSE_8MHZ

#define DISPLAY_RESX 240
#define DISPLAY_RESY 320
#define DISPLAY_COLOR_MODE DMA2D_OUTPUT_RGB565
#define DISPLAY_LEGACY_HEADER "displays/st7789v.h"
#define DISPLAY_PANEL_LHS200KB_IF21 1
#define DISPLAY_I8080_16BIT_DW 1

// #define DISPLAY_PANEL_INIT_SEQ lhs200kb_if21_init_seq
// #define DISPLAY_PANEL_ROTATE lhs200kb_if21_rotate
// #define TRANSFORM_TOUCH_COORDS lhs200kb_if21_transform_touch_coords

#define BACKLIGHT_PWM_FREQ 50000
#define BACKLIGHT_PWM_TIM TIM1
#define BACKLIGHT_PWM_TIM_CLK_EN __HAL_RCC_TIM1_CLK_ENABLE
#define BACKLIGHT_PWM_TIM_AF GPIO_AF1_TIM1
#define BACKLIGHT_PWM_TIM_OCMODE TIM_OCMODE_PWM2
#define BACKLIGHT_PWM_TIM_CHANNEL TIM_CHANNEL_1
#define BACKLIGHT_PWM_TIM_CCR CCR1
#define BACKLIGHT_PWM_PIN GPIO_PIN_7
#define BACKLIGHT_PWM_PORT GPIOA
#define BACKLIGHT_PWM_PORT_CLK_EN __HAL_RCC_GPIOA_CLK_ENABLE

#define I2C_COUNT 1
#define I2C_INSTANCE_0 I2C2
#define I2C_INSTANCE_0_CLK_EN __HAL_RCC_I2C2_CLK_ENABLE
#define I2C_INSTANCE_0_CLK_DIS __HAL_RCC_I2C2_CLK_DISABLE
#define I2C_INSTANCE_0_PIN_AF GPIO_AF4_I2C2
#define I2C_INSTANCE_0_SDA_PORT GPIOB
#define I2C_INSTANCE_0_SDA_PIN GPIO_PIN_11
#define I2C_INSTANCE_0_SDA_CLK_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define I2C_INSTANCE_0_SCL_PORT GPIOB
#define I2C_INSTANCE_0_SCL_PIN GPIO_PIN_10
#define I2C_INSTANCE_0_SCL_CLK_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define I2C_INSTANCE_0_RESET_REG &RCC->APB1RSTR
#define I2C_INSTANCE_0_RESET_BIT RCC_APB1RSTR_I2C2RST
#define I2C_INSTANCE_0_EV_IRQHandler I2C2_EV_IRQHandler
#define I2C_INSTANCE_0_ER_IRQHandler I2C2_ER_IRQHandler
#define I2C_INSTANCE_0_EV_IRQn I2C2_EV_IRQn
#define I2C_INSTANCE_0_ER_IRQn I2C2_ER_IRQn
#define I2C_INSTANCE_0_GUARD_TIME 0

#define TOUCH_PANEL_LHS200KB_IF21 1
#define TOUCH_SENSITIVITY 0x06
#define TOUCH_I2C_INSTANCE 0
#define TOUCH_RST_PORT GPIOC
#define TOUCH_RST_PIN GPIO_PIN_5
#define TOUCH_INT_PORT GPIOC
#define TOUCH_INT_PIN GPIO_PIN_4
#define TOUCH_ON_PORT GPIOB
#define TOUCH_ON_PIN GPIO_PIN_8

#define SD_DETECT_PORT GPIOB
#define SD_DETECT_PIN GPIO_PIN_0
#define SD_ENABLE_PORT GPIOE
#define SD_ENABLE_PIN GPIO_PIN_1

#define GPIO_1_PORT GPIOC
#define GPIO_1_PIN GPIO_PIN_1
#define GPIO_2_PORT GPIOC
#define GPIO_2_PIN GPIO_PIN_6
#define GPIO_3_PORT GPIOC
#define GPIO_3_PIN GPIO_PIN_7

#define BTN_POWER_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define BTN_POWER_PORT GPIOE
#define BTN_POWER_PIN GPIO_PIN_0

#endif  //_TREZOR_T3W1_H
