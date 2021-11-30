/**
 * TODO:
 * - display minutes and seconds
 * - blink led when time out
 * - show timegoal for 5 seconds after button press
 * - require button press to clear blinking and reset
 * - hold down both to display battery
 */

#include "Arduino.h"
#include "Wire.h"
#include "Qduino.h"
#include <SoftwareSerial.h>
#include <EEPROM.h>

#define COLON 4

#define INPUT_PIN_INCR 12
#define INPUT_PIN_DECR 9
#define SOFTWARE_TX 8
#define SOFTWARE_RX 7

enum State {
  COUNTING_DOWN = 0,
  SETTING_TIME_GOAL = 1,
  AT_ZERO = 2
};

qduino q;


State state = COUNTING_DOWN;
int timeGoalMinutes = 1;
char tempString[10];
const int timerGoalAddr = 0;
unsigned long goalTime = 0;
unsigned long startedSettingTimeGoalMillis = 0;

SoftwareSerial s7s(SOFTWARE_RX, SOFTWARE_TX);

void setup() {
  State state = COUNTING_DOWN;

  s7s.begin(9600);
  s7s.write(0x76);  // Clear display
  s7s.write(0x77);
  s7s.write(1<<COLON);

  q.setup();
  pinMode(INPUT_PIN_INCR, INPUT_PULLUP);
  pinMode(INPUT_PIN_DECR, INPUT_PULLUP);

  timeGoalMinutes = EEPROM.read(timerGoalAddr);
  timeGoalMinutes = 1;
  goalTime = millis() + (timeGoalMinutes * 1000L * 60);
}

void readGoalAdjustment() {
  int incrVal = digitalRead(INPUT_PIN_INCR);
  int decrVal = digitalRead(INPUT_PIN_DECR);
  if (incrVal == HIGH || decrVal == HIGH) {
    q.setRGB(PURPLE);
  }
  if (incrVal == LOW || decrVal == LOW) {
    state = SETTING_TIME_GOAL;
    startedSettingTimeGoalMillis = millis();
    q.setRGB(YELLOW);
  }

  if (incrVal == LOW) {
    timeGoalMinutes++;
  }

  if (decrVal == LOW) {
    timeGoalMinutes--;
  }

  EEPROM.write(timerGoalAddr, timeGoalMinutes);
}

void displayTimeLeft() {
  unsigned long timeLeftMillis = goalTime - millis();

  if (millis() > goalTime) {
    goalTime = millis() + (timeGoalMinutes * 1000L * 60);
    timeLeftMillis = goalTime;
  }


  int totalSeconds = timeLeftMillis / 1000;
  int seconds = totalSeconds % 60;
  int minutes = totalSeconds / 60;

  sprintf(tempString, "%02d%02d", minutes, seconds);
  s7s.print(tempString);

}

void displayTimeGoal() {
  sprintf(tempString, "%4d", timeGoalMinutes);
  s7s.print(tempString);
}

void writeDisplayCustom() {
  sprintf(tempString, "%d", state);
  s7s.print(tempString);
}

void loop() {
  switch (state) {
    case COUNTING_DOWN:
      displayTimeLeft();
      break;

    case SETTING_TIME_GOAL:
      displayTimeGoal();
      if (millis() > (startedSettingTimeGoalMillis + (1000 * 5))) {
        state = COUNTING_DOWN;
      }
      break;

    case AT_ZERO:
      break;

    default:
      writeDisplayCustom();
      break;
  }

  readGoalAdjustment();
}
