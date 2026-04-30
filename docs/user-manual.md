# JSY-MK- 194 GSingle-phasebi-directionalenergymetermodule

## 1 .Productdescription

## 1. 1 Introduction

## 1. 2 Functionalfeatures

## 1. 3 Technicalparameters

## 2 .Application

## 2. 1 Appearanceandinstallation

## 2. 2 Interfacedefinition

## 2. 3 Applicationinstructions

## 2. 4 Electricenergymeasurementfunction

## 3 .Modbusregister

## 4 .Modbuscommunicationprotocol

## 5 .Precautions

# 1 .Productdescription

## 1. 1 、 Introduction

JSY-MK- 194 G single-phasetwo-channel electric energy measurement moduleisa
single-phaseACparametermeasurementproductthathighlyintegratesmeasurementand
digitalcommunicationtechnologyandcancompleteelectricenergymeasurement,collection
andtransmission.Itcanaccuratelymeasuretwo-channelsingle-phaseACvoltage,Electrical
parameterssuchascurrent,power,powerfactor,frequency,electricity,etc., 1 - wayTTL
interface,completelyisolatedcircuit,smallsize,simpleinterface,canbeeasilyembedded
intovariousequipmentthatneedtomeasurepowerconsumption,andhasexcellentvalue
formoney.
JSY-MK- 194 Gsingle-phasetwo-channelelectricenergymeteringmodulecanbewidely
used in energy-saving transformation , new energy charging piles, electric power,
communications,railways,transportation,environmentalprotection,petrochemicals,steel
andotherindustriestomonitorthecurrentandPowerconsumption.

## 1. 2 、 FunctionalFeatures

```
1. 2. 1 .Collectsingle-phaseandtwo-channelalternatingcurrentparameters,including
voltage,current,power,factor,frequency,electricenergyandotherelectrical
parameters.
```

```
1. 2. 2 .Adoptspecialmeasurementchip,effectivevaluemeasurementmethod,high
measurementaccuracy.
1. 2. 3 .With 1 - wayTTLcommunicationinterface,compatiblewith 5 V/ 3. 3 Vinterface.
1. 2. 4 .ThecommunicationprotocoladoptsModbus-RTU,whichhasgoodcompatibility
andfacilitatesprogramming.
1. 2. 5 .Highisolationvoltage,withstandvoltageuptoDC 3000 V.
```

## 1. 3 、 TechnicalParameters

```
1. 3. 1 SinglephaseACinput
1 )Voltagerange: 1 ~ 300 V(canbecustomized).
2 )Currentrange: 20 mA~ 50 A, 20 mA~ 100 A(canbecustomized).
3 )Signalprocessing:usingspecialmeasurementchip, 24 - bitADsampling.
4 )Overloadcapacity: 1. 2 timesthecurrentrangeissustainable,and 1. 5 timesthe
voltagerangeisnotdamaged.
5 )Inputimpedance:voltagechannel> 1 kΩ/V.
1. 3. 2 CommunicationInterface
1 )Interfacetype:oneTTLcommunicationinterface,compatiblewith 5 V/ 3. 3 V.
2 )Communicationprotocol:Modbus-RTUprotocol.
3 )Dataformat:Thedefaultis"n, 8 , 1 ","e, 8 , 1 ","o, 8 , 1 ","n, 8 , 2 "canbeset.
4 )Baudrate:thedefaultis 4800 bps, 9600 bps, 19200 bpsand 38400 bpscanbeset.
1. 3. 3 Measurementoutputdata
Formultipleelectricalparameterssuchasvoltage,current,power,electricenergy,
powerfactor,frequency,etc.,seetheModbusdataregisterlist.
1. 3. 4 Measurementaccuracy
Voltage,current,power,electricity:lessthan± 1. 0 %.
1. 3. 5 Isolation
Thepowersupplyundertestandthepowersupplyareisolatedfromeachother.
theisolationwithstandvoltageis 3000 VDC.
1. 3. 6 Powersupply
1 )DCsinglepowersupply 3. 3 ~ 5 Vpowersupply,powerconsumption 10 mA.
1. 3. 7 Workingenvironment
1 )Workingtemperature:- 20 ～+ 60 °C.
2 )Relativehumidity: 5 ~ 95 %,nocondensation(at 40 °C).
```

```
3 )Altitude: 0 ~ 3000 meters.
4 )Environment:Noexplosive,corrosivegasesandconductivedust,nosignificant
shaking,vibrationandimpact.
1. 3. 8 Temperaturedrift:≤ 100 ppm/°C.
1. 3. 9 Installationmethod:PCBwelding.
1. 3. 10 Modulesize: 65 * 48 * 28 mm(length*width*height)
```

# 2 .Application

## 2. 1 、 Appearanceandinstallation

## 2. 2 、 Interfacedefinition

```
2. 2. 1 Pindescription
logo characteristic Functiondescription
UL Livewire Measuredlivewireconnectionport
UN Neutralwire Measuredneutralwireconnectionport
VCC Powersupply+ Meteringmodulepowersupplypin,normalapplicationrange:
3. 3 V- 5 V
GND Powersupply- Powerground
RX Input ModuleTTLreceivingpin
TX Output ModuleTTLsendingpin
```

## 2. 3 、 ApplicationNotes

```
Pleaserefertotheabovediagramforcorrectwiringaccordingtoproductspecifications
andmodels.Makesuretodisconnectallsignalsourcesbeforewiringtoavoiddangerand
damagetotheequipment.Aftercheckingtoconfirmthatthewiringiscorrect,turnonthe
powerandtest.
Afterthepoweristurnedon,the"indicatorlight"isalwayson,andduring
communication,the"indicatorlight"flashessynchronouslyduringcommunicationdata
transmission.
Whentheproductleavesthefactory,itissettothedefaultconfiguration:addressNo. 1 ,
baudrate 4800 bps,dataformat"n, 8 , 1 ",dataupdaterateis 330 msonce,andthe
transformationratiois 1.
YoucanusetheJSY-MK- 194 Gseriesproducttestingsoftwareweprovidetochangethe
settingsofproductparametersandgeneraltestingoftheproduct.
```

## 2. 4 、 Electricenergymeasurementfunction

```
Canprovidesingle-phasevoltage,current,power,powerfactor,frequency,activeenergy
andotherparameters.
Theelectricitydataisa 4 - byteunsignednumber.Itwillnotoverflowfor 10 consecutive
yearsandthedatawillbesavedwhenthepoweristurnedoff.
```

```
Figure 2. 1 Wiringdiagram
```

# 3 、 JSY-MK- 194 G Modbusregisterlist

## Table 1 :Systemconfigurationparameterregisteraddress(Function

## code: 03 H-read, 10 H-write)

```
Number Definiti
on
```

```
Register
address
```

```
Read/
write
Description
```

### 1

```
IDand
baud
rate
```

### 0004 H

```
Read/
write
```

```
Defaultvalue: 0105 H
(defaultIDis 01 H,defaultcommunicationformatis
8 ,N, 1 , 4800 bps)
explain：
HighbyteisID,itcanbesetas 1 to 255.
Lowbyteisbaud
rate, 3 - 1200 bps, 4 - 2400 bps, 5 — 4800 bps, 6 — 9600 bps, 7 -
19200 bps, 8 - 38400 bps
```

## Table 2 :Systemparameter （ Functioncode: 03 H-read,readonly ）

```
Number
Definition
Register
address
Read/write Description
1 Model 1 0000 H Read Valueis 0194 H
2 Model 2 0001 H Read Reserved
3 Voltage
range
```

```
0002 H Read Valueis 250 (V)
```

```
4 Current
range
```

```
0003 H Read Valueis 800 （ 800 / 10 = 80 A）
```

## Table 3 :Measuring electrical parameter register （ Function

## code: 03 H-read, 10 H-write ）

```
Number Definition Register
address
```

```
Read/write Bytelength Description
```

```
1 Firstchannel
voltage
```

```
0048 H Read 4 Unsigned,Value=DATA/ 1000
0 （V）
```

```
2 Firstchannel
current
```

```
0049 H Read 4 Unsigned,Value=DATA/ 1000
0 （A）
3 Firstchannel
activepower
```

```
004 AH Read 4 Unsigned,Value=DATA/ 1000
0 （W）
4 Positive
activeenergy
offirst
channel
```

```
004 BH Read/Write 4 Unsigned,Value=DATA/ 1000
0 （kWh）
```

```
5 Firstchannel
powerfactor
```

```
004 CH Read 4 Unsigned,Value=DATA/ 1000
```

```
6 Negative
activeenergy
offirst
channel
```

```
004 DH Read/Write 4 Unsigned,Value=DATA/ 1000
0 （kWh）
```

```
7 Power
direction 004 EH
```

Read (^4) Firstbyte(firstchannel):
00 - positive, 01 - negative.
Secondbyte(second
channel): 00 - positive, 01 - nega
tive
8 Frequency
004 FH
Read 4 Unsigned,Value==DATA/ 10
0 (Hz)
9 Second
channel
voltage

### 0050 H

```
Read 4 Unsigned,Value=DATA/ 100
00 （V）
```

```
10 Second
channel
current
```

### 0051 H

```
Read 4 Unsigned,Value=DATA/ 100
00 （A）
```

```
11 Second
channel
activepower
```

### 0052 H

```
Read 4 Unsigned,Value=DATA/ 100
00 （W）
```

```
12 Positive
activeenergy
ofsecond
channel
```

```
0053 H Read/Write 4 Unsigned,Value=DATA/ 1000
0 （kWh）
```

```
13 Second
channel
powerfactor
```

```
0054 H Read 4 Unsigned,Value=DATA/ 1000
```

```
14 Negative
activeenergy
ofsecond
channel
```

```
0055 H Read/Write 4 Unsigned,Value=DATA/ 1000
0 （kWh）
```

# 四、 ModubusCommunicationProtocol

Thisinstrument providesaserial asynchronoushalf-duplexRS 485 communication
interface,usingthestandardMODBUS-RTUprotocol,andvariousdatainformationcanbe
transmittedonthecommunicationline.Upto 255 networkinstrumentscanbeconnectedto
onelineatthesametime.Eachnetworkinstrumentcansetitscommunicationaddress.The
communicationconnectionshoulduseashieldedtwistedpairwithacoppermesh,andthe
wirediametershouldnotbelessthan 0. 5 mm^2 .Whenwiring,communicationlinesshouldbe
keptawayfromstrongcurrentcablesorotherstrongelectricfieldenvironments.
TheMODBUSprotocoladoptsthemaster-slaveresponsecommunicationconnection
methodononecommunicationline.First,thesignalfromthehostcomputerisaddressedto
aterminaldevice(slave)withauniqueaddress.Then,theresponsesignalfromtheterminal
deviceistransmittedtothehostintheoppositedirection,thatis,thesignalistransmitted
alongaseparatecommunicationline.Allcommunicationdatastreamsaretransmittedin
oppositedirections (half-duplex operating mode). The MODBUSprotocol only allows
communicationbetweenthehost(PC,PLC,etc.)andterminaldevices,butdoesnotallow
dataexchangebetweenindependentterminaldevices.Inthisway,eachterminaldevicewill
notoccupythecommunicationlinewhentheyareinitialized,butislimitedtoresponding.
Querysignalarrivingatthismachine.

```
Hostquery:Thequerymessageframeincludesdeviceaddress,functioncode,data
informationcode,andcheckcode.Theaddresscodeindicatestheslavedevicetobeselected.
thefunctioncodetellstheselectedslavedevicewhatfunctionitwantstoperform.For
example,functioncode 03 or 04 requirestheslavedevicetoreadregistersandreturntheir
contents.thedatasegmentcontainstherequirementsoftheslavedevice.Anyadditional
informationtoperformthefunction,thecheckcodeisusedtoverifythecorrectnessofa
frameofinformation,theslavedeviceprovidesamethodtoverifywhetherthemessage
contentiscorrect,itusesthecalibrationruleofCRC 16.
Slaveresponse:Iftheslavedevicegeneratesanormalresponse,theresponsemessage
containstheslaveaddresscode,functioncode,datainformationcodeandCRC 16 checkcode.
Datainformationcodesincludedatacollectedfromthedevice:likeregistervaluesorstatus.
Ifanerroroccurs,weagreethattheslavemachinewillnotrespond.
Wespecifythecommunicationdataformatusedinthisinstrument:bitsperbyte( 1 start
bit, 8 databits,oddorevenparityornoparity, 1 or 2 stopbits).
Thestructureofthedataframe,thatis,themessageformat:
Deviceaddress functioncode datasegment CRC 16 checkcode
1 byte 1 byte Nbytes 2 bytes(lowbytefirst)
```

```
Deviceaddress:Itconsistsofonebyte.Theaddressofeachterminaldevicemustbeunique.
Onlytheaddressedterminalwillrespondtothecorrespondingquery.
Functioncode:tellstheaddressedterminalwhatfunctiontoperform.Thefollowingtablelists
thefunctioncodessupportedbythisseriesofinstrumentsandtheirfunctions.
```

Functioncode Function
03 H Readthevalueofoneormoreregisters
10 H Writethevalueofoneormoreregisters
01 H Readtheoutputstatusofrelay 1
05 H Writetheoutputstatusofrelay 1
Datasegment:Containsthedatarequiredbytheterminaltoperformspecificfunctions
orthedatacollectedwhentheterminalrespondstoqueries.Thecontentofthesedatamay
benumericalvalues,referenceaddressesorsettingvalues.
Checkcode:CRC 16 occupiestwobytesandcontainsa 16 - bitbinaryvalue.TheCRC
valueiscalculatedbythetransmittingdeviceandthenappendedtothedataframe.The
receivingdevicerecalculatestheCRCvaluewhenreceivingthedataandthencomparesit
withthevalueinthereceivedCRCfield.Ifthetwovaluesarenotequal,anerroroccurs.
mistake.
TheprocessofgeneratingaCRC 16 is:
( 1 )Preseta 16 - bitregisterto 0 FFFFH(all 1 s),calledCRCregister.
( 2 )PerformXORoperationonthe 8 bitsofthefirstbyteinthedataframeandthelow
byteintheCRCregister,andstoretheresultbackintotheCRCregister.
( 3 )ShifttheCRCregisteronebittotheright,fillthehighestbitwith 0 ,shiftoutthe
lowestbitanddetectit.
( 4 )Ifthelowestbitis 0 :repeatthethirdstep(nextshift).ifthelowestbitis 1 :performan
XORoperationontheCRCregisterandapresetfixedvalue( 0 A 001 H).
( 5 )Repeatsteps 3 and 4 until 8 shifts.Inthisway,acompleteeightbitsareprocessed.
( 6 )Repeatsteps 2 to 5 toprocessthenexteightbitsuntilallbytesareprocessed.
( 7 )ThefinalvalueoftheCRCregisteristhevalueofCRC 16.

## MODBUS-RTUcommunicationprotocolexample

**4. 1 .Functioncode 0 x 03 :Readmultipleregisters**
Example:Thehostwantstoread 2 slaveregisterdatawithaddress 01 andstartaddress
0048 H.
Hostsends: 010300480002 CRC
AddressfunctioncodestartingaddressdatalengthCRCcode
Slaveresponse: 01030412455668 CRC

```
AddressfunctioncodereturnsthenumberofbytesRegisterdata 1 Registerdata 2 CRCcode
4. 2 .Functioncode 0 x 10 :Writemultipleregisters
Example:Thehostwantstosave 0000 , 0000 totheslaveregisterwithaddresses 000 C, 000 D
(slaveaddresscodeis 0 x 01 )
Hostsends: 0110000 C 00020400000000 F 3 FA
Addressfunctioncodestartingaddressnumberofwriteregistersbytecountsaveddata 12
CRCcode
Slaveresponse: 0110000 C 000281 CB
AddressfunctioncodestartingaddresswriteregisternumberCRCcode
4. 3 .Description
TheregisterintheMODBUS-RTUcommunicationprotocolrefersto 16 bits(ie 2 bytes),and
thehigh-orderbitisfirst.
Whensettingparameters,becarefulnottowriteillegaldata(thatis,datavaluesthatexceed
thedatarangelimit).
Theerrorcodeformatreturnedbytheslaveisasfollows:
Addresscode: 1 byte
Functioncode: 1 byte(thehighestbitis 1 )
Errorcode: 1 byte
CRC: 2 bytes
Theresponsereturnsthefollowingerrorcode:
81 :Illegalfunctioncode,thatis,thereceivedfunctioncodemoduledoesnotsupportit.
82 :Readingorwritingillegaldataaddress,thatis,thedatalocationexceedsthereadableor
writableaddressrangeofthemodule.
83 :Illegaldatavalue,thatis,thedatavaluesentbythemodulereceivedbythehostexceeds
thedatarangeofthecorrespondingaddress.
```

## 4. 4 .Exampleofcommandanalysis

```
4. 4. 1 Readelectricalparameterinstructions(takethemoduleaddressas 0 x 01 asanexample):
Senddata: 01030048000 E 4418 (read 14 registersstartingfrom 0048 H)
Receiveddata: 01033800241 EF 90005 FEA 3058 D 376800004 F 38000003 E 80000
3 DF 1000000000000138900241 EF 90005 FEB 9058 B 671800004 ECA 000003 E 8
```

00003 DA 2 C 19 E(Theredpartisthedatacorrespondingtothe 0048 Hstartregister),

```
4. 4. 2 Clearenergycommand(takemoduleaddress 0 x 01 asanexample):
Senddata: 0110000 C 00020400000000 F 3 FA
Receivedata: 0110000 C 000281 CB
```

# 5 .Precautions

1 ) Payattentiontotheauxiliarypowerinformationontheproductlabel.Donotconnect
thewrongauxiliarypowerlevelandpolarityoftheproduct,otherwisetheproductmay
bedamaged.
2 ) Pleaserefertothediagramforcorrectwiringaccordingtoproductspecificationsand
models.Makesuretodisconnectallsignalsourcesandpowerbeforewiringtoavoid
dangeranddamagetotheequipment.Aftercheckingtoconfirmthatthewiringis
correct,turnonthepowerandtest.
3 ) ThevoltagecircuitorthesecondarycircuitofthePTmustnotbeshort-circuited.
4 ) WhenthereiscurrentontheprimarysideoftheCT,itisstrictlyforbiddentoopenthe
secondarycircuitoftheCT.itisstrictlyforbiddentomakeliveconnectionsorunplug
terminals.

```
5 ) Whenusingtheproductinanenvironmentwithstrongelectromagneticinterference,
pleasepayattentiontotheshieldingoftheinputandoutputsignallines.
6 ) Wheninstalledinacentralizedmanner,theminimuminstallationintervalshouldnotbe
lessthan 10 mm.
7 ) Thisseriesofproductsdoesnothavealightningprotectioncircuitinside.Whenthe
inputand outputfeedersofthemoduleareexposed to harsh outdoorweather
conditions,lightningprotectionmeasuresshouldbetaken.
8 ) Pleasedonotdamageormodifytheproduct'slabelorlogo,anddonotdisassembleor
modifytheproduct,otherwiseourcompanywillnolongerprovide"threeguarantees"
(guaranteedreplacement,guaranteedreturn,andguaranteedrepair)serviceforthe
product.
```

```
Manufacturer:ShenzhenJiansiyanTechnologiesCo.,Ltd.
OnlineTechnicalSupportStaff:
+ 8618675534520 (Mr.Jahleel)
+ 8618665924579 (Mr.Jimmy)
E-mail:jsykj@outlook.com
Web:www.jsypowermeter.com
Address: 901 ,Building 1 ,TaijialeTechnologyIndustrialPark,TongguanRoad,
TianliaoCommunity,YutangStreet,GuangmingDistrict,Shenzhen,Guangdong,
518100 ,China.
```
