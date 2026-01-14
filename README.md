# cerc

an f-stop darkroom timer

## hardware

### input devices

- SW1 BURN_BTN (16mm NO MOM)
- SW2 PRINT_BTN (22mm NO MOM)
- SW3 FS_INTERVAL_KNOB (RS1010 1P4T)
- SW4 SW_FOCUS (MTS-102)
- SW5 SW_SFL (MTS-102)
- SW6 SW_MODE (MTS-102)
- ENC1 FS_INCREMENT_KNOB (KY-040)

### output devices

- DS1 DISPLAY (HT16K33, 7-SEG 3-BIT Anode 0.8")
- BZ1 BUZZER_PWM (16Ω Integrated Passive Buzzer 3V 40mA)
- K1 ENL_PULSE (SSR)
- K2 ENL_HOLD (MR)
- K3 SFL_CTRL (SSR)

## workflow

The timer exclusively works in f-stops, i.e. the display does not display seconds. The timer counts down as per the interval selected with FS_INTERVAL_KNOB. e.g. FS_INTERVAL_KNOB=1/3, therefore 5.5, 5.2, 4.9, 4.6, 4.3, and so on...

| State | DS1 (3-bit Display) | BZ1 (Buzzer) | K1/K2 (Enlarger) | K3 (Safelight) |
| :--- | :--- | :--- | :--- | :--- |
| **Idle / Selecting** | f-stops (e.g. `5.5`) | Silent | OFF | **ON** |
| **Focus Mode** | f-stops (counting) | Silent | **ON** | OFF |
| **Exposure Delay** | No change | High Chirp | OFF | OFF |
| **Exposing** | Countdown (e.g. `4.2`) | Silent | **ON** | OFF -> 1s delay |
| **Test Segment Pause** | Next stop (blink) | Two Chirps | OFF | **ON** |
| **Finished** | **out** | Two Chirps | OFF | **ON** |
| **Burn Mode Entry** | **b0.0** | Short Chirp | OFF | No Change |
| **Invalid Action** | No change | **ERR_BUZZ** | No change | No Change |
| **Safety Lock** | **hot** | Long Pulse | **Forced OFF** | **ON** |

SW_FOCUS puts the enlarger on for up to 3 minutes, where after it will automatically turn off for safety reasons (overheating). The timer will not be usable and the display will display « hot » for 3 minutes thereafter, until the enlarger can be put on again. An override is possible by double pressing the BURN_BTN. Additionally, it is forbidden to press the PRINT_BTN whilst SW_FOCUS is closed as the lamp would not turn off after the expoure (since it is manually forced on).

SW_SFL will put the safelight on/off. if it is off, that manually overrides automatic on/off ALWAYS, as per table 1.1 (for ra-4 development).

### modes

there are 3 modes:

#### BURN_MODE

BURN_MODE allows the user to burn/dodge a print relative to the exposure used during exposure. Such that, if one wishes to burn for +2/3 stops, one does not need to calculate anything, but merely press BURN_BTN, and add stops to the exposure time used just before during the initial exposure using the FS_INTERVAL_KNOB and FS_INCREMENT_KNOB as usual. The burn time is displays as b[X].[Y]. Therefore, BURN_MODE may only be entered when the screen diplays « out » by pressing the BURN_BTN, otherwise, nothing will happen. To Exit BURN_MODE, just press the BURN_BTN again.

e.g. You just finished an exposure of 5.5 stops, however, you decide to burn it for another half stop. You press BURN_BTN to enter BURN_MOD, where the display shows « b0.0 ». Increase stops using the FS_INCREMENT_KNOB, as per the interval defined by the FS_INTERVAL_KNOB. The display will adjust accordingly. Once you are content with the selected burn time, such as « b0.5 », you print using the PRINT_BTN.

#### PRINT_MODE

PRINT_MODE indicates that the upon pressing, the PRINT_BTN initiate an exposure for the amount of f-stops selected using the FS_INCREMENT_KNOB, which will be displayed clearly on the screen as [X].[Y]. Double pressing the PRINT_BTN cancels an exposure. This mode is entered when SW_MODE is closed (to the left).

#### TEST_MODE

TEST_MODE allows the users to create test strips upon pressing the PRINT_BTN. Double pressing the PRINT_BTN cancels the exposure. The difference between each strip's being the interval defined by FS_INTERVAL_KNOB. The quantity of test strips is always 5 (for now, maybe i will make it so you can change it using FS_INCREMENT_KNOB somehow). The base exposure (e.g. 5.5) is set using the FS_INCRMENT_KNOB, the first strip will be the `base exposure-  2 * FS_INTERVAL_KNOB` (e.g. 5.5-2*0.5=4.5), the second would naturally be 5.0, the base exposure is the middle exposure, then 6.0, 6.5. BUZZER_PWM buzzes at the end of ever test exposure, where the user is expected to repress the PRINT_BTN to start the next exposure in the test. Thismode is entered when SW_MODE is open (to the right).
