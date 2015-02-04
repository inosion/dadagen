package org.soqqo.rdg;

public class DataTypes
{
  public static enum Address
  {
    StreetAddress, CityTown, StateCountyProvince, Country, HouseName, StreetNumber, StreetName, StreetType;
  }

  public static enum Contact
  {
    Email, Phone;
  }

  public static enum DateAndTime
  {
    Now, Random;
  }

  public static enum HumanAttributes
  {
    EyeColour, HairColour, Height, Weight, Age, DateOfBirth, Sex, SexMale, SexFemale;
  }

  public static enum Name
  {
    Name, Lastname, Initial, Title, MaleName, FemaleName;
  }

  public static enum NumbersAndLetters
  {
    AutoIncrement, Range, Random;
  }
}
