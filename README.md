# toki lukin pi telo kule

ilo ni li pali e SVG pi sitelen pona.\
nasin lawa en nasin sitelen pi ilo ni li lon anpa.

## o jo

mi kepeken e ubuntu pi linux.\
mi kepepen e ilo apt. sina ken kepeken ala.\
sina kepeken windows la sina kepeken ala e lipu sona ni.

o wile jo e ilo python:\
```sudo apt install python3.12-dev```

<!--   -->

o luka lili te ```Releases``` to.\
o kama jo te ```tlptk.tar.gz``` to tan GitHub.\
o pali e kulupu tan ijo ```tlptk.tar.gz```.

## o kepeken

### lawa lukin

ilo kulupu pi ilo sona li jo e ilo te ```tlptk``` to.\
o open kepeken te ```./tlptk``` to.\
ilo ni li pali sin pi jan ala e lipu SVG. sina pini la sina luka lili e te ```X``` to.

![ui](./ui.png)

pana weka: ```tlptk/project/test/output.svg```

lipu sin SVG li weka e lipu majuna SVG.

### nasin lawa

o sina te ```python3 writer.py --help``` to.\
```python3 writer.py -w 200 -n './test/sin.tlptk' -p 1400 -s 600```

sina wile ante e sitelen lukin. sina kepeken te ```sudo apt install inkscape``` to.

o sina te ```python3 convert.py --help``` to.\
```python3 convert.py -n ./nimi.svg -l png -p 1000```

nasin lon pi ilo sona pi pana open li sama e pana weka.

## nasin sitelen

sitelen wan pi sitelen weka li te ```_``` to.

```tlptk
ilo ni li toki lukin pi telo kule
mi jan sin_sitelen pona li pona tawa mi a
mi wile e ni la jan pi toki pona li kama mute a
```

![sin](./sin.svg)

nimi poki li kepeken e te ```[]``` to.

```tlptk
nimi mi li [jan pi pipi luka]
```

![nimi](./nimi.svg)

nimi ante li kepeken e te ```*``` to.

```tlptk
sewi*mute*
```

sitelen lasina li kepeken e te ```''``` to.

```tlptk
'toki pona'
```

nimi te e nimi to li kepeken e te ```""``` to.

```tlptk
"toki pona"
```
