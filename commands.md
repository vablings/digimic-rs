

- SPI -> DBG ON/OFF
- MAC? -> //Returns device mac address
- KEY? -> KEY1
- CALK / CALK1 / CALK0 //Typing this will display -Cal- on the screen and freeze the device?
- MEN? -> MEN02F0000E //possibly a ptr?
- TST -> //Cycles thought various test menus such as showing full display illuminated
- IN/MM -> //Changes the device between imperial and metric measurement
- MCUID -> MSP6726 // Returns the microcontroller uid
- VBAT -> VBAT 3179 // Gives the battery voltage in format 0.000v
- BLI <on/off> // toggles the screen blinking function
- CFGBT? -> CFG BT SIMPLE //prints out the current bluetooth mode
- OUT <on/off> //toggles continous output mode
- TMP? -> TMP +091.70 //No idea what this does
- ID -> SY 306 // Shows the device id
- LCAL -> 00.00.2000 // Last calibrated date i would assume
- NCAL -> 00.00.2000 // Next calibration date i would also assume.
- DIS // Just seems to turn the segment display off
- OUTR? -> OUTR 0050
- SBY //Sends the unit into standby mode
- UNI <off/on> //unsure 
- CAL <on/off> // Enter calibration mode, gives you an extra dp

| nth | paddr       | vaddr      | string           |
|-----|-------------|------------|------------------|
| 164 |  0x00008415 | 0x00008415 | ART?             |
| 165 |  0x0000841a | 0x0000841a | BAT?             |
| 166 |  0x0000841f | 0x0000841f | BLI1             |
| 167 |  0x00008424 | 0x00008424 | BLION            |
| 168 |  0x0000842a | 0x0000842a | BLI0             |
| 169 |  0x0000842f | 0x0000842f | BLIOFF           |
| 170 |  0x00008436 | 0x00008436 | BLI?             |
| 171 |  0x0000843b | 0x0000843b | ADVI?            |
| 172 |  0x00008441 | 0x00008441 | ADVI             |
| 173 |  0x0000844e | 0x0000844e | BTON             |
| 174 |  0x00008453 | 0x00008453 | BTOFF            |
| 175 |  0x0000845d | 0x0000845d | BTRST            |
| 176 |  0x00008463 | 0x00008463 | MAC?             |
| 177 |  0x0000846c | 0x0000846c | CFGBT?           |
| 178 |  0x00008473 | 0x00008473 | CFGBTSIMPLE      |
| 179 |  0x0000847f | 0x0000847f | CFGBTPAIR        |
| 180 |  0x00008489 | 0x00008489 | CFGBTHID         |
| 181 |  0x00008492 | 0x00008492 | CFGCNT1          |
| 182 |  0x0000849a | 0x0000849a | CFGCNT0          |
| 183 |  0x000084a2 | 0x000084a2 | CFGCNTON         |
| 184 |  0x000084ab | 0x000084ab | CFGCNTOFF        |
| 185 |  0x000084b5 | 0x000084b5 | CFGCNT?          |
| 186 |  0x000084bd | 0x000084bd | CFGSVRAUTO       |
| 187 |  0x000084c8 | 0x000084c8 | CFGSVRSINGLE     |
| 188 |  0x000084d5 | 0x000084d5 | CFGSVR?          |
| 189 |  0x000084dd | 0x000084dd | CFGCLMD?         |
| 190 |  0x000084e6 | 0x000084e6 | CFGCLMD0         |
| 191 |  0x000084ef | 0x000084ef | CFGCLMDOFF       |
| 192 |  0x000084fa | 0x000084fa | CFGCLMD1         |
| 193 |  0x00008503 | 0x00008503 | CFGCLMDON        |
| 194 |  0x0000850d | 0x0000850d | BUF1             |
| 195 |  0x00008512 | 0x00008512 | BUF0             |
| 196 |  0x00008517 | 0x00008517 | BUFON            |
| 197 |  0x0000851d | 0x0000851d | BUFOFF           |
| 198 |  0x00008524 | 0x00008524 | BUF?             |
| 199 |  0x00008529 | 0x00008529 | CFGHID           |
| 200 |  0x00008530 | 0x00008530 | FACCFGHID        |
| 201 |  0x0000853a | 0x0000853a | PBAT?            |
| 202 |  0x00008540 | 0x00008540 | VBAT?            |
| 203 |  0x00008546 | 0x00008546 | FACCFGFMIN       |
| 204 |  0x00008551 | 0x00008551 | CAL1             |
| 205 |  0x00008556 | 0x00008556 | CAL0             |
| 206 |  0x0000855b | 0x0000855b | CALON            |
| 207 |  0x00008561 | 0x00008561 | CALOFF           |
| 208 |  0x00008568 | 0x00008568 | CAL?             |
| 209 |  0x0000856d | 0x0000856d | CFGFMIN          |
| 210 |  0x00008575 | 0x00008575 | CFGPREFIX1       |
| 211 |  0x00008580 | 0x00008580 | CFGPREFIX0       |
| 212 |  0x0000858b | 0x0000858b | CFGPREFIXON      |
| 213 |  0x00008597 | 0x00008597 | CFGPREFIXOFF     |
| 214 |  0x000085a4 | 0x000085a4 | CFGPREFIX?       |
| 215 |  0x000085af | 0x000085af | CORON            |
| 216 |  0x000085b5 | 0x000085b5 | COROFF           |
| 217 |  0x000085bc | 0x000085bc | COR?             |
| 218 |  0x000085c1 | 0x000085c1 | CORREF           |
| 219 |  0x000085c8 | 0x000085c8 | CORRST           |
| 220 |  0x000085d3 | 0x000085d3 | TCORON           |
| 221 |  0x000085da | 0x000085da | TCOROFF          |
| 222 |  0x000085e2 | 0x000085e2 | TCOR?            |
| 223 |  0x000085e8 | 0x000085e8 | TCORREF          |
| 224 |  0x000085f0 | 0x000085f0 | TCORRST          |
| 225 |  0x000085f8 | 0x000085f8 | TCOR             |
| 226 |  0x000085fd | 0x000085fd | ECHO1            |
| 227 |  0x00008603 | 0x00008603 | ECHO0            |
| 228 |  0x00008609 | 0x00008609 | ECHOON           |
| 229 |  0x00008610 | 0x00008610 | ECHOOFF          |
| 230 |  0x00008618 | 0x00008618 | ECHO?            |
| 231 |  0x0000861e | 0x0000861e | ECO1             |
| 232 |  0x00008623 | 0x00008623 | ECO0             |
| 233 |  0x00008628 | 0x00008628 | ECOON            |
| 234 |  0x0000862e | 0x0000862e | ECOOFF           |
| 235 |  0x00008635 | 0x00008635 | ECO?             |
| 236 |  0x0000863a | 0x0000863a | ERS1             |
| 237 |  0x0000863f | 0x0000863f | ERS0             |
| 238 |  0x00008644 | 0x00008644 | ERSON            |
| 239 |  0x0000864a | 0x0000864a | ERSOFF           |
| 240 |  0x00008651 | 0x00008651 | ERS?             |
| 241 |  0x00008656 | 0x00008656 | FACOFF           |
| 242 |  0x0000865d | 0x0000865d | FACPW1978ART     |
| 243 |  0x0000866a | 0x0000866a | FACPW1978FMINON  |
| 244 |  0x0000867a | 0x0000867a | FACPW1978FMINOFF |
| 245 |  0x0000868b | 0x0000868b | FACPW1978FMIN    |
| 246 |  0x00008699 | 0x00008699 | FACPW1978IN      |
| 247 |  0x000086a5 | 0x000086a5 | FACPW1978MM      |
| 248 |  0x000086b1 | 0x000086b1 | FACPW1978UNI     |
| 249 |  0x000086be | 0x000086be | FACPW1978RES     |
| 250 |  0x000086cb | 0x000086cb | FACRST           |
| 251 |  0x000086d2 | 0x000086d2 | FACPW1978SN      |
| 252 |  0x000086de | 0x000086de | FACPW1978RSTIN   |
| 253 |  0x000086ed | 0x000086ed | FACPW1978RSTMM   |
| 254 |  0x000086fc | 0x000086fc | FACPW1978RSTPRE  |
| 255 |  0x00008714 | 0x00008714 | FLA?             |
| 256 |  0x00008719 | 0x00008719 | FMINOFF          |
| 257 |  0x00008721 | 0x00008721 | FMINON           |
| 258 |  0x00008728 | 0x00008728 | FMIN1            |
| 259 |  0x0000872e | 0x0000872e | FMIN0            |
| 260 |  0x00008734 | 0x00008734 | FMIN?            |
| 261 |  0x0000873a | 0x0000873a | HOLD1            |
| 262 |  0x00008740 | 0x00008740 | HOLD0            |
| 263 |  0x00008746 | 0x00008746 | HOLDON           |
| 264 |  0x0000874d | 0x0000874d | HOLDOFF          |
| 265 |  0x00008755 | 0x00008755 | HOLD?            |
| 266 |  0x00008765 | 0x00008765 | KEY1             |
| 267 |  0x0000876a | 0x0000876a | KEY0             |
| 268 |  0x0000876f | 0x0000876f | KEYON            |
| 269 |  0x00008775 | 0x00008775 | KEYOFF           |
| 270 |  0x0000877c | 0x0000877c | KEY?             |
| 271 |  0x00008781 | 0x00008781 | LCAL?            |
| 272 |  0x00008787 | 0x00008787 | LCAL             |
| 273 |  0x0000878c | 0x0000878c | NCAL?            |
| 274 |  0x00008792 | 0x00008792 | NCAL             |
| 275 |  0x00008797 | 0x00008797 | CALA1            |
| 276 |  0x0000879d | 0x0000879d | CALA0            |
| 277 |  0x000087a3 | 0x000087a3 | CALAON           |
| 278 |  0x000087aa | 0x000087aa | CALAOFF          |
| 279 |  0x000087b2 | 0x000087b2 | CALA?            |
| 280 |  0x000087b8 | 0x000087b8 | CALK1            |
| 281 |  0x000087be | 0x000087be | CALK0            |
| 282 |  0x000087c4 | 0x000087c4 | CALKON           |
| 283 |  0x000087cb | 0x000087cb | CALKOFF          |
| 284 |  0x000087d3 | 0x000087d3 | CALK?            |
| 285 |  0x000087dd | 0x000087dd | LIN?             |
| 286 |  0x000087e6 | 0x000087e6 | MEN?             |
| 287 |  0x000087ef | 0x000087ef | MCUID?           |
| 288 |  0x000087fe | 0x000087fe | OUT1             |
| 289 |  0x00008803 | 0x00008803 | OUT0             |
| 290 |  0x00008808 | 0x00008808 | OUTON            |
| 291 |  0x0000880e | 0x0000880e | OUTOFF           |
| 292 |  0x00008815 | 0x00008815 | OUT?             |
| 293 |  0x0000881a | 0x0000881a | OUTR?            |
| 294 |  0x00008820 | 0x00008820 | OUTR             |
| 295 |  0x00008825 | 0x00008825 | PAR?             |
| 296 |  0x0000882a | 0x0000882a | PREON            |
| 297 |  0x00008830 | 0x00008830 | PREOFF           |
| 298 |  0x00008851 | 0x00008851 | SET?             |
| 299 |  0x0000885e | 0x0000885e | STO1             |
| 300 |  0x00008863 | 0x00008863 | STO0             |
| 301 |  0x00008868 | 0x00008868 | STOON            |
| 302 |  0x0000886e | 0x0000886e | STOOFF           |
| 303 |  0x00008875 | 0x00008875 | STO?             |
| 304 |  0x0000887a | 0x0000887a | SVR1             |
| 305 |  0x0000887f | 0x0000887f | SVR0             |
| 306 |  0x00008884 | 0x00008884 | SVR?             |
| 307 |  0x00008889 | 0x00008889 | SVRON            |
| 308 |  0x0000888f | 0x0000888f | SVROFF           |
| 309 |  0x00008896 | 0x00008896 | TBL?             |
| 310 |  0x0000889b | 0x0000889b | TMP?             |
| 311 |  0x000088a4 | 0x000088a4 | UNI1             |
| 312 |  0x000088a9 | 0x000088a9 | UNI0             |
| 313 |  0x000088ae | 0x000088ae | UNION            |
| 314 |  0x000088b4 | 0x000088b4 | UNIOFF           |
| 315 |  0x000088bb | 0x000088bb | UNI?             |
| 316 |  0x000088c0 | 0x000088c0 | VER?             |
| 317 |  0x000088c5 | 0x000088c5 | PRE              |
