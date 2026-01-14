# cerc

an f-stop darkroom timer

## hardware

### input devices

SW1 BURN_BTN
SW2 PRINT_BTN
SW3 FS_INTERVAL_KNOB
SW4 SW_FOCUS
SW5 SW_SFL
SW6 SW_MODE
ENC1 FS_INCREMENT_KNOB

### output devices

DS1 3BIT-DISPLAY
BZ1 BUZZER_PWM
K1 ENL_PULSE
K2 ENL_HOLD
K3 SFL_CTRL

## software/logic/workflow?

3BIT-DISPLAY will display f stops, which will countdown as per the interval selected by FS_INTERVAL_KNOB.
e.g. FS_INTERVAL_KNOB = 1/3， therefore 5.5, 5.2, 4.9, 4.6, 4.3, so on...

SW_FOCUS will put the enlarger on (it automatically flips off after 3 minutes for safety purposes maybe? the display will write "hot" and leave it like that for 3 minutes to cool down. override is possibly by pressing BURN_BTN twice). ERR_BUZZ will play if you try to press PRINT_BTN while SW_FOCUS is closed. (the lamp wouldnt turn off after exposure).

SW_SFL will put the safelight on/off. if it is off, that manually overrides automatic on/off ALWAYS, as per table 1.1 (for ra-4 development).

### modes

there are 3 modes:

PRINT_MODE
TEST_MODE
BURN_MODE

#### BURN_MODE

BURN_MODE, which always takes priority, is entered upon pressing BURN_BTN, given that you are not currently in BURN_MODE. If you press BURN_BTN while in BURN_MODE, you exit it. While in BURN_MODE, you use FS_INCREMENT_KNOB to add extra stops to the just exposed exposure, that are displayed as b[X].[Y]. If you press BURN_BTN at any point not after exposure (when display displays "out" (i.e. from "Time's Out)), the buzzer iwll play ERR_BUZZ.

e.g. Ended exposure of 5.5 stops. You decide to burn another part by 0.3 stops. You press BURN_BTN to enter mode, which displays 0 additional stops. you use FS_INCREMENT_KNOB to add stops to expose, which is decided by FS_INTERVAL_KNOB. then you click PRINT_BTN to start, as usual.

#### PRINT_MODE

SW_MODE IS CLOSED.
PRINT_BTN simply initialises the exposures as per the amount of f stops selected using the FS_INCREMENT_KNOB. Double clicking PRINT_BTN cancels.

#### TEST_MODE

SW_MODE IS OPEN.
PRINT_BTN initialises the process. Based on the interval. For example, interval set to 1/2, base exposure is 4. it will start from 3, 3.5, 4, 4.5, 5, where the middle exposure is the initially set base exposure. BUZZER_PWM buzzes at the end of ever test exposure, where the user is expected to repress the PRINT_BTN to start the next exposure in the test.

PRINT_BTN, regardless of MODE, always turns off the SFL_CTRL, safely waits 1 seconds, then begins to print.


| State | DS1 Display | BZ1 Buzzer | K1/K2 (Enlarger) | K3 (Safelight) |

| :--- | :--- | :--- | :--- | :--- |
| **Idle / Selecting** | f-stops (e.g. `5.5`) | Silent | OFF | **ON** |
| **Focus Mode** | f-stops (counting) | Silent | **ON** | OFF |
| **Exposure Delay** | No change | High Chirp | OFF | OFF |
| **Exposing** | Countdown (e.g. `4.2`) | Silent | **ON** | OFF |
| **Test Segment Pause** | Next stop (blink) | Two Chirps | OFF | **ON** |
| **Finished** | **out** | Two Chirps | OFF | **ON** |
| **Burn Mode Entry** | **b0.0** | Short Chirp | OFF | No Change |
| **Invalid Action** | No change | **ERR_BUZZ** | No change | No Change |
| **Safety Lock** | **hot** | Long Pulse | **Forced OFF** | **ON** |
