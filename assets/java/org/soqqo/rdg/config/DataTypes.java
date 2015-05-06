package org.soqqo.rdg.config;

public class DataTypes {
    public static enum Address {
        StreetAddress, CityTown, StateCountyProvince, Country, HouseName, StreetNumber, StreetName, StreetType;
    }

    public static enum Contact {
        Email, Phone;
    }

    public static enum DateAndTime {
        Now, Random, Sequential;
    }
    
    public static enum DTConfig { 
        RoundToYear, RoundToMonth, RoundToDay, RoundToHour, RoundToMinute, RoundToSeconds;
    }

    public static enum HumanAttributes {
        EyeColour, HairColour, Height, Weight, Age, DateOfBirth, Sex, SexMale, SexFemale;
    }

    public static enum Name {
        Firstname, Lastname, Initial, Title, MaleFirstname, Femalename;
    }

    public static enum NumbersListsAndLetters {
        AutoIncrement, Range, Random;
    }

    /**
     * Used when we just want a random object created for a field, and the random object is itself a random config generated one.
     * @author r007b
     *
     */
    public static enum SubObject {
        ObjectGeneration, SubList
    }
}
