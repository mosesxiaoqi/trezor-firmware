#ifndef _STM32F429I_DISC1_H
#define _STM32F429I_DISC1_H

#define HSE_8MHZ

#define DISPLAY_RESX 240
#define DISPLAY_RESY 320
#define DISPLAY_COLOR_MODE DMA2D_OUTPUT_RGB565
#define DISPLAY_LEGACY_HEADER "displays/ltdc.h"

#define I2C_COUNT 1
#define I2C_INSTANCE_0 I2C3
#define I2C_INSTANCE_0_CLK_EN __HAL_RCC_I2C3_CLK_ENABLE
#define I2C_INSTANCE_0_CLK_DIS __HAL_RCC_I2C3_CLK_DISABLE
#define I2C_INSTANCE_0_PIN_AF GPIO_AF4_I2C3
#define I2C_INSTANCE_0_SDA_PORT GPIOC
#define I2C_INSTANCE_0_SDA_PIN GPIO_PIN_9
#define I2C_INSTANCE_0_SDA_CLK_EN __HAL_RCC_GPIOC_CLK_ENABLE
#define I2C_INSTANCE_0_SCL_PORT GPIOA
#define I2C_INSTANCE_0_SCL_PIN GPIO_PIN_8
#define I2C_INSTANCE_0_SCL_CLK_EN __HAL_RCC_GPIOA_CLK_ENABLE
#define I2C_INSTANCE_0_RESET_REG &RCC->APB1RSTR
#define I2C_INSTANCE_0_RESET_BIT RCC_APB1RSTR_I2C3RST
#define I2C_INSTANCE_0_EV_IRQHandler I2C3_EV_IRQHandler
#define I2C_INSTANCE_0_ER_IRQHandler I2C3_ER_IRQHandler
#define I2C_INSTANCE_0_EV_IRQn I2C3_EV_IRQn
#define I2C_INSTANCE_0_ER_IRQn I2C3_ER_IRQn
#define I2C_INSTANCE_0_GUARD_TIME 0

#define TOUCH_I2C_INSTANCE 0

#endif  //_STM32F429I_DISC1_H
