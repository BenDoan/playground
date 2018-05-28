function on(pin)
    gpio.mode(pin, gpio.OUTPUT)
    gpio.write(pin, gpio.HIGH)
end

function off(pin)
    gpio.mode(pin, gpio.OUTPUT)
    gpio.write(pin, gpio.LOW)
end
