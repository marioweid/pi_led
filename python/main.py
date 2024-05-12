import RPi.GPIO as GPIO
from time import sleep

GPIO.setwarnings(False)
GPIO.setmode(GPIO.BOARD)   # Use physical pin numbering
# GPIO.setup(16, GPIO.OUT, initial=GPIO.LOW)   # Set pin 36 to be an output pin and set initial value to low (off)
# GPIO.setup(18, GPIO.OUT, initial=GPIO.LOW)   # Set pin 38 to be an output pin and set initial value to low (off)
GPIO.setup(22, GPIO.OUT, initial=GPIO.LOW)   # Set pin 40 to be an output pin and set initial value to low (off)

red = 22
green = None
blue = None

test_pin = 22
try:

    GPIO.output(test_pin, GPIO.HIGH) # on
    sleep(5)
    GPIO.output(test_pin, GPIO.LOW)  # off
except:
    GPIO.cleanup()
    print("Pi exploded and room is on fire. Call 911 !!!")
finally:
    GPIO.cleanup()