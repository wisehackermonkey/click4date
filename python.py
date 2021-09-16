# 20180307
import datetime
import pyperclip
import time
d = datetime.datetime.now()

day = str(d.day).rjust(2,"0")
month = str(d.month).rjust(2,"0")
year = d.year 

pyperclip.copy("{}{}{}".format(year,month,day) )
