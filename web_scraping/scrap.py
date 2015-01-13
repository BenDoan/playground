import urllib2
from BeautifulSoup import BeautifulSoup

colleges = ["ACCT", "ANTH", "AE", "ART", "AVN", "BIOL", "BMI", "BLST", "BRCT", "BSAD", "CHEM", "CIVE", "CIST", "COMM", "CEEN", "CSCI", "CONE", "CNST", "COOP", "COUN", "CRCJ", "ECON", "EDL", "EDUC", "EMGT", "ENGR", "ENGL", "ENVE", "ENVN", "FNBK", "FSCI", "FLNG", "FREN", "GEOG", "GEOL", "GERM", "GERO", "GDRH", "HED", "HPER", "HIST", "HONR", "ILUN", "ITIN", "IASC", "ISQA", "INST", "JOUR", "LLS", "LAWS", "MGMT", "MKT", "MATH", "MTCH", "MENG", "MILS", "MUS", "NAMS", "PHIL", "PE", "PEA", "PHYS", "PSCI", "PSYC", "PA", "RELU", "RLS", "RELI", "SOWK", "SOC", "SPAN", "SPED", "SPCH", "STAT", "TED", "THEA", "UBNS", "WGST"]
for college in colleges:
    URL = "http://www.unomaha.edu/class-search/?term=1145&session=&subject={}&catalog_nbr=&career=&instructor=&class_start_time=&class_end_time=&location=&special=&instruction_mode=".format(college)
    r = urllib2.urlopen(URL).read()
    soup = BeautifulSoup(r)
    courses = soup.findAll(attrs={"class":"dotted-bottom"})
    for course in courses:
        if course.find("h2") != None:
            print course.find("h2").string
        if course.find("p") != None:
            print course.find("p").string

        sections = course.findAll(attrs={'class':'span6'})
        print len(sections)
