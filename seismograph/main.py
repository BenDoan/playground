import pyb

log = open('/sd/accel.csv', 'a+')
light1 = pyb.LED(4)
light2 = pyb.LED(1)

def write_button():
    log.write("mark\n")
    log.flush()

    light2.on()
    pyb.delay(500)
    light2.off()
switch = pyb.Switch()
switch.callback(write_button)

rtc = pyb.RTC()

accel = pyb.Accel()
pyb.delay(200) # wait for accel to initialize

light1.on()

while True:
    x, y, z = accel.filtered_xyz()

    dt = rtc.datetime()
    time = "{0}-{1}-{2}T{3}:{4}".format(*dt)

    log.write("{},{},{},{},{}\n".format(pyb.millis(), time, x, y, z))
    log.flush()

    pyb.delay(250)

log.close()
light1.off()
