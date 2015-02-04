package org.soqqo.datagen;

import java.util.ArrayList;
import java.util.Date;
import java.util.List;

import junit.framework.Assert;

import org.junit.Test;
import org.soqqo.rdg.RandomDataGenerator;
import org.soqqo.rdg.config.DataTypes.DTConfig;
import org.soqqo.rdg.config.DataTypes.Name;
import org.soqqo.rdg.config.GenConfig;

public class TestRandomness {

    @Test
    public void testStringArrayGenerator() {
        RandomDataGenerator rdg = new RandomDataGenerator();

        GenConfig randomConfig = GenConfig();
        randomConfig
        rdg.generateStringList(


        )
    }

    @Test
    public void testRandomNameGeneration() {

        RandomDataGenerator rdg = new RandomDataGenerator();
        List<Person> randomPersons = rdg.generateList(50, new GenConfig().name(Name.Firstname, "firstname").name(Name.Lastname, "lastname"), Person.class);

        Assert.assertTrue(randomPersons.size() == 50);

        for (Person p : randomPersons) {
            System.out.println(p);
        }
    }

    @Test
    public void testRandomList() {

        RandomDataGenerator rdg = new RandomDataGenerator();
        List<String> list = RandomUtil.stringToList("Apple,Pear,Orange");
        List<Person> randomPersons = rdg.generateList(7, new GenConfig().nextFromStringList("firstname", "Apple,Pear,Orange"), Person.class);

        Assert.assertTrue(randomPersons.size() == 7);

        int i = 0;

        for (Person p : randomPersons) {
            if (i == list.size()) {
                i = 0;
            }
            Assert.assertTrue(p.getFirstname().equals(list.get(i++)));
            Assert.assertTrue(list.contains(p.getFirstname()));
            System.out.println("!matched " + p.getFirstname());
        }
    }

    @Test
    public void testRandomObject() {

        RandomDataGenerator rdg = new RandomDataGenerator();
        List<Person> randomPersons = rdg.generateList(50,

                new GenConfig()
                    .name(Name.Firstname, "firstname")
                    .name(Name.Lastname, "lastname")
                    .randomObject(
                             new GenConfig()
                                 .randomFromStringList("age", "eighteen months, eight, twenty two, thirty one, thirty two, thiry three, thirty four, five, six, seven")
                             , "myAge"
                             , Age.class
                     )
                 ,Person.class);

        Assert.assertTrue(randomPersons.size() == 50);

        for (Person p : randomPersons) {
            Assert.assertTrue(p.getMyAge() != null);
            System.out.println(p.getMyAge().getAge());
        }
    }

    @Test
    public void testfromAList() {

        RandomDataGenerator rdg = new RandomDataGenerator();
        List<String> strings = new ArrayList<String>();
        for (int i = 0; i < 10; i++) {
            strings.add(RandomUtil.randomFromStringSeparatedList("foo,bar,baz,and,some,mioe-moe,goo,go"));
        }

        List<Person> randomPersons = rdg.generateList(50, new GenConfig().nextFromList("lastname", strings).name(Name.MaleFirstname, "firstname"), Person.class);

        Assert.assertTrue(randomPersons.size() == 50);

        for (Person p : randomPersons) {
            System.out.println(p);
            Assert.assertTrue("foo,bar,baz,and,some,mioe-moe,goo,go".contains(p.getLastname()));

        }
    }

    @Test
    public void testNowDate() {

        RandomDataGenerator rdg = new RandomDataGenerator();
        List<Person> randomPersons = rdg.generateList(10, new GenConfig().dateTimeNow("created"), Person.class);

        Assert.assertTrue(randomPersons.size() == 10);

        Date now = new Date();
        Date previous10Seconds = new Date(now.getTime() - 10000);

        for (Person p : randomPersons) {
            // make sure it was created in the last 10 seconds
            System.out.println("now=" + now + " created=" + p.getCreated());
            Assert.assertTrue((p.getCreated().equals(now) || p.getCreated().before(now)) && p.getCreated().after(previous10Seconds));

        }
    }

    @Test
    public void testRandomDate() {

        RandomDataGenerator rdg = new RandomDataGenerator();
        List<Person> randomPersons = rdg.generateList(10, new GenConfig().dateTimeRandom("created"), Person.class);

        Assert.assertTrue(randomPersons.size() == 10);

        Date now = new Date();

        for (Person p : randomPersons) {
            System.out.println("now=" + now + " created=" + p.getCreated());
            Assert.assertTrue((p.getCreated() != null));

        }
    }

    @Test
    public void testNRandomRangeDate() {

        RandomDataGenerator rdg = new RandomDataGenerator();
        Date endDate = new Date();
        Date beginDate = new Date(endDate.getTime() - 1000000000000L);

        List<Person> randomPersons = rdg.generateList(10, new GenConfig().dateTimeRandom("created", beginDate, endDate), Person.class);

        Assert.assertTrue(randomPersons.size() == 10);

        for (Person p : randomPersons) {
            System.out.println("begin=" + beginDate + " created=" + p.getCreated() + " end=" + endDate);
            Assert.assertTrue((p.getCreated().equals(endDate) || p.getCreated().before(endDate)) && p.getCreated().after(beginDate));

        }
    }

    @Test
    public void testNowRestrictedDate() {

        RandomDataGenerator rdg = new RandomDataGenerator();
        Date now = new Date();

        List<Person> randomPersons = null;

        randomPersons = rdg.generateList(1, new GenConfig().dateTimeNow("created", DTConfig.RoundToSeconds), Person.class);
        now = new Date();
        Assert.assertTrue(randomPersons.size() == 1);

        for (Person p : randomPersons) {
            System.out.println("RoundToSeconds = now=" + now + " created=" + p.getCreated());
            Assert.assertTrue(p.getCreated().getTime() != now.getTime());
            Assert.assertTrue(p.getCreated().getSeconds() == now.getSeconds());
            Assert.assertTrue(p.getCreated().getMinutes() == now.getMinutes());
            Assert.assertTrue(p.getCreated().getHours() == now.getHours());
            Assert.assertTrue(p.getCreated().getDate() == now.getDate());
            Assert.assertTrue(p.getCreated().getMonth() == now.getMonth());
            Assert.assertTrue(p.getCreated().getYear() == now.getYear());

        }

        randomPersons = rdg.generateList(1, new GenConfig().dateTimeNow("created", DTConfig.RoundToMinute), Person.class);
        now = new Date();
        Assert.assertTrue(randomPersons.size() == 1);

        for (Person p : randomPersons) {
            System.out.println("RoundToMinute = now=" + now + " created=" + p.getCreated());
            Assert.assertTrue(p.getCreated().getTime() != now.getTime());
            Assert.assertTrue(p.getCreated().getSeconds() != now.getSeconds());
            Assert.assertTrue(p.getCreated().getMinutes() == now.getMinutes());
            Assert.assertTrue(p.getCreated().getHours() == now.getHours());
            Assert.assertTrue(p.getCreated().getDate() == now.getDate());
            Assert.assertTrue(p.getCreated().getMonth() == now.getMonth());
            Assert.assertTrue(p.getCreated().getYear() == now.getYear());

        }

        randomPersons = rdg.generateList(1, new GenConfig().dateTimeNow("created", DTConfig.RoundToHour), Person.class);
        now = new Date();
        Assert.assertTrue(randomPersons.size() == 1);

        for (Person p : randomPersons) {
            System.out.println("RoundToHour = now=" + now + " created=" + p.getCreated());
            Assert.assertTrue(p.getCreated().getTime() != now.getTime());
            Assert.assertTrue(p.getCreated().getSeconds() != now.getSeconds());
            Assert.assertTrue(p.getCreated().getMinutes() != now.getMinutes());
            Assert.assertTrue(p.getCreated().getHours() == now.getHours());
            Assert.assertTrue(p.getCreated().getDate() == now.getDate());
            Assert.assertTrue(p.getCreated().getMonth() == now.getMonth());
            Assert.assertTrue(p.getCreated().getYear() == now.getYear());

        }

        randomPersons = rdg.generateList(1, new GenConfig().dateTimeNow("created", DTConfig.RoundToDay), Person.class);
        now = new Date();
        Assert.assertTrue(randomPersons.size() == 1);

        for (Person p : randomPersons) {
            System.out.println("RoundToDay = now=" + now + " created=" + p.getCreated());
            Assert.assertTrue("milliseconds are equal",p.getCreated().getTime() != now.getTime() );
            Assert.assertTrue("seconds are not 0",p.getCreated().getSeconds() == 0);
            Assert.assertTrue("minutes are not 0",p.getCreated().getMinutes()==0);
            Assert.assertTrue("hours are not 0",p.getCreated().getHours() ==0);
            Assert.assertTrue("days are not equal",p.getCreated().getDate() == now.getDate());
            Assert.assertTrue("month not matched", p.getCreated().getMonth() == now.getMonth());
            Assert.assertTrue("year is not the same",p.getCreated().getYear() == now.getYear());

        }

        randomPersons = rdg.generateList(1, new GenConfig().dateTimeNow("created", DTConfig.RoundToMonth), Person.class);
        now = new Date();
        Assert.assertTrue(randomPersons.size() == 1);

        for (Person p : randomPersons) {
            System.out.println("RoundToMonth = now=" + now + " created=" + p.getCreated());
            Assert.assertTrue("milliseconds are equal",p.getCreated().getTime() != now.getTime() );
            Assert.assertTrue("seconds are not 0",p.getCreated().getSeconds() == 0);
            Assert.assertTrue("minutes are not 0",p.getCreated().getMinutes()==0);
            Assert.assertTrue("hours are not 0",p.getCreated().getHours() ==0);
            Assert.assertTrue(p.getCreated().getDate() != now.getDate());
            Assert.assertTrue(p.getCreated().getMonth() == now.getMonth());
            Assert.assertTrue(p.getCreated().getYear() == now.getYear());

        }

        randomPersons = rdg.generateList(1, new GenConfig().dateTimeNow("created", DTConfig.RoundToYear), Person.class);
        now = new Date();
        Assert.assertTrue(randomPersons.size() == 1);

        for (Person p : randomPersons) {
            System.out.println("RoundToYear = now=" + now + " created=" + p.getCreated());
            Assert.assertTrue("milliseconds are equal",p.getCreated().getTime() != now.getTime() );
            Assert.assertTrue("seconds are not 0",p.getCreated().getSeconds() == 0);
            Assert.assertTrue("minutes are not 0",p.getCreated().getMinutes()==0);
            Assert.assertTrue("hours are not 0",p.getCreated().getHours() ==0);
            Assert.assertTrue(p.getCreated().getDate() != now.getDate());
            Assert.assertTrue(p.getCreated().getMonth() != now.getMonth());
            Assert.assertTrue(p.getCreated().getYear() == now.getYear());

        }

    }

}
