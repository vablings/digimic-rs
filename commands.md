

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

=====================================================================
RAW DUMP
=====================================================================
164  0x00008415 0x00008415 4   5            ascii   ART?
165  0x0000841a 0x0000841a 4   5            ascii   BAT?
166  0x0000841f 0x0000841f 4   5            ascii   BLI1
167  0x00008424 0x00008424 5   6            ascii   BLION
168  0x0000842a 0x0000842a 4   5            ascii   BLI0
169  0x0000842f 0x0000842f 6   7            ascii   BLIOFF
170  0x00008436 0x00008436 4   5            ascii   BLI?
171  0x0000843b 0x0000843b 5   6            ascii   ADVI?
172  0x00008441 0x00008441 4   5            ascii   ADVI
173  0x0000844e 0x0000844e 4   5            ascii   BTON
174  0x00008453 0x00008453 5   6            ascii   BTOFF
175  0x0000845d 0x0000845d 5   6            ascii   BTRST
176  0x00008463 0x00008463 4   5            ascii   MAC?
177  0x0000846c 0x0000846c 6   7            ascii   CFGBT?
178  0x00008473 0x00008473 11  12           ascii   CFGBTSIMPLE
179  0x0000847f 0x0000847f 9   10           ascii   CFGBTPAIR
180  0x00008489 0x00008489 8   9            ascii   CFGBTHID
181  0x00008492 0x00008492 7   8            ascii   CFGCNT1
182  0x0000849a 0x0000849a 7   8            ascii   CFGCNT0
183  0x000084a2 0x000084a2 8   9            ascii   CFGCNTON
184  0x000084ab 0x000084ab 9   10           ascii   CFGCNTOFF
185  0x000084b5 0x000084b5 7   8            ascii   CFGCNT?
186  0x000084bd 0x000084bd 10  11           ascii   CFGSVRAUTO
187  0x000084c8 0x000084c8 12  13           ascii   CFGSVRSINGLE
188  0x000084d5 0x000084d5 7   8            ascii   CFGSVR?
189  0x000084dd 0x000084dd 8   9            ascii   CFGCLMD?
190  0x000084e6 0x000084e6 8   9            ascii   CFGCLMD0
191  0x000084ef 0x000084ef 10  11           ascii   CFGCLMDOFF
192  0x000084fa 0x000084fa 8   9            ascii   CFGCLMD1
193  0x00008503 0x00008503 9   10           ascii   CFGCLMDON
194  0x0000850d 0x0000850d 4   5            ascii   BUF1
195  0x00008512 0x00008512 4   5            ascii   BUF0
196  0x00008517 0x00008517 5   6            ascii   BUFON
197  0x0000851d 0x0000851d 6   7            ascii   BUFOFF
198  0x00008524 0x00008524 4   5            ascii   BUF?
199  0x00008529 0x00008529 6   7            ascii   CFGHID
200  0x00008530 0x00008530 9   10           ascii   FACCFGHID
201  0x0000853a 0x0000853a 5   6            ascii   PBAT?
202  0x00008540 0x00008540 5   6            ascii   VBAT?
203  0x00008546 0x00008546 10  11           ascii   FACCFGFMIN
204  0x00008551 0x00008551 4   5            ascii   CAL1
205  0x00008556 0x00008556 4   5            ascii   CAL0
206  0x0000855b 0x0000855b 5   6            ascii   CALON
207  0x00008561 0x00008561 6   7            ascii   CALOFF
208  0x00008568 0x00008568 4   5            ascii   CAL?
209  0x0000856d 0x0000856d 7   8            ascii   CFGFMIN
210  0x00008575 0x00008575 10  11           ascii   CFGPREFIX1
211  0x00008580 0x00008580 10  11           ascii   CFGPREFIX0
212  0x0000858b 0x0000858b 11  12           ascii   CFGPREFIXON
213  0x00008597 0x00008597 12  13           ascii   CFGPREFIXOFF
214  0x000085a4 0x000085a4 10  11           ascii   CFGPREFIX?
215  0x000085af 0x000085af 5   6            ascii   CORON
216  0x000085b5 0x000085b5 6   7            ascii   COROFF
217  0x000085bc 0x000085bc 4   5            ascii   COR?
218  0x000085c1 0x000085c1 6   7            ascii   CORREF
219  0x000085c8 0x000085c8 6   7            ascii   CORRST
220  0x000085d3 0x000085d3 6   7            ascii   TCORON
221  0x000085da 0x000085da 7   8            ascii   TCOROFF
222  0x000085e2 0x000085e2 5   6            ascii   TCOR?
223  0x000085e8 0x000085e8 7   8            ascii   TCORREF
224  0x000085f0 0x000085f0 7   8            ascii   TCORRST
225  0x000085f8 0x000085f8 4   5            ascii   TCOR
226  0x000085fd 0x000085fd 5   6            ascii   ECHO1
227  0x00008603 0x00008603 5   6            ascii   ECHO0
228  0x00008609 0x00008609 6   7            ascii   ECHOON
229  0x00008610 0x00008610 7   8            ascii   ECHOOFF
230  0x00008618 0x00008618 5   6            ascii   ECHO?
231  0x0000861e 0x0000861e 4   5            ascii   ECO1
232  0x00008623 0x00008623 4   5            ascii   ECO0
233  0x00008628 0x00008628 5   6            ascii   ECOON
234  0x0000862e 0x0000862e 6   7            ascii   ECOOFF
235  0x00008635 0x00008635 4   5            ascii   ECO?
236  0x0000863a 0x0000863a 4   5            ascii   ERS1
237  0x0000863f 0x0000863f 4   5            ascii   ERS0
238  0x00008644 0x00008644 5   6            ascii   ERSON
239  0x0000864a 0x0000864a 6   7            ascii   ERSOFF
240  0x00008651 0x00008651 4   5            ascii   ERS?
241  0x00008656 0x00008656 6   7            ascii   FACOFF
242  0x0000865d 0x0000865d 12  13           ascii   FACPW1978ART
243  0x0000866a 0x0000866a 15  16           ascii   FACPW1978FMINON
244  0x0000867a 0x0000867a 16  17           ascii   FACPW1978FMINOFF
245  0x0000868b 0x0000868b 13  14           ascii   FACPW1978FMIN
246  0x00008699 0x00008699 11  12           ascii   FACPW1978IN
247  0x000086a5 0x000086a5 11  12           ascii   FACPW1978MM
248  0x000086b1 0x000086b1 12  13           ascii   FACPW1978UNI
249  0x000086be 0x000086be 12  13           ascii   FACPW1978RES
250  0x000086cb 0x000086cb 6   7            ascii   FACRST
251  0x000086d2 0x000086d2 11  12           ascii   FACPW1978SN
252  0x000086de 0x000086de 14  15           ascii   FACPW1978RSTIN
253  0x000086ed 0x000086ed 14  15           ascii   FACPW1978RSTMM
254  0x000086fc 0x000086fc 15  16           ascii   FACPW1978RSTPRE
255  0x00008714 0x00008714 4   5            ascii   FLA?
256  0x00008719 0x00008719 7   8            ascii   FMINOFF
257  0x00008721 0x00008721 6   7            ascii   FMINON
258  0x00008728 0x00008728 5   6            ascii   FMIN1
259  0x0000872e 0x0000872e 5   6            ascii   FMIN0
260  0x00008734 0x00008734 5   6            ascii   FMIN?
261  0x0000873a 0x0000873a 5   6            ascii   HOLD1
262  0x00008740 0x00008740 5   6            ascii   HOLD0
263  0x00008746 0x00008746 6   7            ascii   HOLDON
264  0x0000874d 0x0000874d 7   8            ascii   HOLDOFF
265  0x00008755 0x00008755 5   6            ascii   HOLD?
266  0x00008765 0x00008765 4   5            ascii   KEY1
267  0x0000876a 0x0000876a 4   5            ascii   KEY0
268  0x0000876f 0x0000876f 5   6            ascii   KEYON
269  0x00008775 0x00008775 6   7            ascii   KEYOFF
270  0x0000877c 0x0000877c 4   5            ascii   KEY?
271  0x00008781 0x00008781 5   6            ascii   LCAL?
272  0x00008787 0x00008787 4   5            ascii   LCAL
273  0x0000878c 0x0000878c 5   6            ascii   NCAL?
274  0x00008792 0x00008792 4   5            ascii   NCAL
275  0x00008797 0x00008797 5   6            ascii   CALA1
276  0x0000879d 0x0000879d 5   6            ascii   CALA0
277  0x000087a3 0x000087a3 6   7            ascii   CALAON
278  0x000087aa 0x000087aa 7   8            ascii   CALAOFF
279  0x000087b2 0x000087b2 5   6            ascii   CALA?
280  0x000087b8 0x000087b8 5   6            ascii   CALK1
281  0x000087be 0x000087be 5   6            ascii   CALK0
282  0x000087c4 0x000087c4 6   7            ascii   CALKON
283  0x000087cb 0x000087cb 7   8            ascii   CALKOFF
284  0x000087d3 0x000087d3 5   6            ascii   CALK?
285  0x000087dd 0x000087dd 4   5            ascii   LIN?
286  0x000087e6 0x000087e6 4   5            ascii   MEN?
287  0x000087ef 0x000087ef 6   7            ascii   MCUID?
288  0x000087fe 0x000087fe 4   5            ascii   OUT1
289  0x00008803 0x00008803 4   5            ascii   OUT0
290  0x00008808 0x00008808 5   6            ascii   OUTON
291  0x0000880e 0x0000880e 6   7            ascii   OUTOFF
292  0x00008815 0x00008815 4   5            ascii   OUT?
293  0x0000881a 0x0000881a 5   6            ascii   OUTR?
294  0x00008820 0x00008820 4   5            ascii   OUTR
295  0x00008825 0x00008825 4   5            ascii   PAR?
296  0x0000882a 0x0000882a 5   6            ascii   PREON
297  0x00008830 0x00008830 6   7            ascii   PREOFF
298  0x00008851 0x00008851 4   5            ascii   SET?
299  0x0000885e 0x0000885e 4   5            ascii   STO1
300  0x00008863 0x00008863 4   5            ascii   STO0
301  0x00008868 0x00008868 5   6            ascii   STOON
302  0x0000886e 0x0000886e 6   7            ascii   STOOFF
303  0x00008875 0x00008875 4   5            ascii   STO?
304  0x0000887a 0x0000887a 4   5            ascii   SVR1
305  0x0000887f 0x0000887f 4   5            ascii   SVR0
306  0x00008884 0x00008884 4   5            ascii   SVR?
307  0x00008889 0x00008889 5   6            ascii   SVRON
308  0x0000888f 0x0000888f 6   7            ascii   SVROFF
309  0x00008896 0x00008896 4   5            ascii   TBL?
310  0x0000889b 0x0000889b 4   5            ascii   TMP?
311  0x000088a4 0x000088a4 4   5            ascii   UNI1
312  0x000088a9 0x000088a9 4   5            ascii   UNI0
313  0x000088ae 0x000088ae 5   6            ascii   UNION
314  0x000088b4 0x000088b4 6   7            ascii   UNIOFF
315  0x000088bb 0x000088bb 4   5            ascii   UNI?
316  0x000088c0 0x000088c0 4   5            ascii   VER?
317  0x000088c5 0x000088c5 4   5            ascii   PRE 

