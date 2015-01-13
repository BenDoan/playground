#!/usr/bin/env python2

from BeautifulSoup import BeautifulSoup
import urllib2

URL = "http://www.unomaha.edu/class-search/"

r = urllib2.urlopen(URL).read()

soup = BeautifulSoup(r)


colleges = ["ACCT", "AERO", "ANTH", "AE", "ARCH", "ART", "AVN", "BIOI", "BIOL", "BMI", "BLST", "BRCT", "BSAD", "CHEM", "CHIN", "CIVE", "CIST", "COMM", "CEEN", "CSCI", "CONE", "CNST", "COOP", "COUN", "CRCJ", "DSGN", "ECON", "EDL", "EDUC", "ELEC", "EMGT", "ENGR", "ENGL", "ENVE", "ENVN", "FNBK", "FSMT", "FSCI", "FLNG", "FREN", "GEOG", "GEOL", "GERM", "GERO", "GDRH", "HED", "HPER", "HIST", "HONR", "HORT", "HUMN", "ILUN", "IPD", "ITIN", "IASC", "ISQA", "IDSG", "INST", "JAPN", "JOUR", "LLS", "LAWS", "MGMT", "MKT", "MFAW", "MATH", "MTCH", "MENG", "MILS", "MUS", "NAMS", "NSCI", "NEUR", "PHIL", "PE", "PEA", "PHYS", "PSCI", "PSYC", "PA", "RELU", "RLS", "RELI", "RUSS", "SSCI", "SOWK", "SOC", "SPAN", "SPED", "SPCH", "STAT", "TED", "THEA", "US", "UBNS", "WGST", "WRWS"]
#colleges = ["CSCI"]
for college in colleges:
    r = urllib2.urlopen("http://www.unomaha.edu/class-search/?term=1141&session=&subject={}&catalog_nbr=&career=&instructor=&class_start_time=&class_end_time=&location=&special=&instruction_mode=".format(college))
    soup = BeautifulSoup(r)
    courses = soup.findAll(attrs={"class":"dotted-bottom"})
    for course in courses:
        if course.find("h2"):
            print course.find("h2").string

        if course.find("p"):
            print course.find("p").string

        sections = course.findAll(attrs={'class':'span6'})
        print len(sections)
        print ""
