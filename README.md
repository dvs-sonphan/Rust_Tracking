### Branch

Branch	|	Mô tả			|
--------|-------------------------------|
main	| Source code chính - release	|
develop	| Branch từ **main** để phát triển các tính năng |
feature	| Branch từ **develop** để code một tính năng cụ thể	|

----------------------------------------------------------------------------------
### Kết nối phần cứng:
- MCU: STM32F303RDT6
- Module GPS: SIM68ML

Kết nối:
STT	|	GPS SIM68ML	|	STM32F303RDT6	|
--------|-----------------------|-----------------------|
1	|	VCC		|	VCC		|
2	|	GND		|	GND		|
3	|	TXD0 (GPIO 2)	| USART2_RX (PIN PA3)	|
4	|	RXD0 (GPIO 3)	| USART2_TX (PIN PA2)	|
5	|	RESET (GPIO 9)	| 	PIN PA1		|
6	|	-		| Led GPS (PIN PC2)	|

DEBUG Serial
```
PA9 - USART1_TX
PA10 - USART1_RX
```

----------------------------------------------------------------------------------
## gps_stm32f3
- Tạo project:
```
cargo new gps_stm32f3
cd gps_stm32f3
code .
```
















