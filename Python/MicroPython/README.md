1. 进入`https://micropython.org/download/GENERIC_S3/`下载固件
2. 烧录固件`esptool --port COM4 write_flash -fm dio 0x00000 .\MicroPython-xxxx.bin`