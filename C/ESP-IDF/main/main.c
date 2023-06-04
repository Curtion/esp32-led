#include <stdio.h>
#include <inttypes.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "driver/gpio.h"

void app_main(void)
{
    gpio_set_direction(GPIO_NUM_1, GPIO_MODE_OUTPUT);
    while (1)
    {
        gpio_set_level(GPIO_NUM_1, 0);
        vTaskDelay(1000 / portTICK_PERIOD_MS);
        gpio_set_level(GPIO_NUM_1, 1);
        vTaskDelay(1000 / portTICK_PERIOD_MS);
    }
}
