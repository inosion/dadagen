package org.soqqo.rdg.config;

import java.util.ArrayList;
import java.util.Date;
import java.util.List;

import org.soqqo.rdg.RandomUtil;
import org.soqqo.rdg.config.DataTypes.DTConfig;
import org.soqqo.rdg.config.DataTypes.DateAndTime;
import org.soqqo.rdg.config.DataTypes.NumbersListsAndLetters;

public class GenConfig {

    private List<DataFieldConfig> fieldConfigurations = new ArrayList<DataFieldConfig>();

    public List<DataFieldConfig> getFieldConfigurations() {
        return fieldConfigurations;
    }

    public void setFieldConfigurations(List<DataFieldConfig> fieldConfigurations) {
        this.fieldConfigurations = fieldConfigurations;
    }

    /**
     * Set a property for a name
     * 
     * @param nameType
     * @param propertyName
     * @param nameParameters
     * @return
     */
    public GenConfig name(DataTypes.Name nameType, String propertyName) {
        this.fieldConfigurations.add(new DataFieldConfig(propertyName, nameType));
        return this;
    }

    public GenConfig address(DataTypes.Address addressType, String propertyName, String addressParameters) {
        this.fieldConfigurations.add(new DataFieldConfig(propertyName, addressType));
        return this;
    }

    public GenConfig randomObject(GenConfig subObjectConfig, String propertyName, Class clazz) {
        this.fieldConfigurations.add(new RandomObjectGeneratingFieldConfig(propertyName, subObjectConfig, clazz));
        return this;
    }

    public GenConfig randomFromStringList(String propertyName, String list) {
        this.fieldConfigurations.add(new ListBasedFieldConfig(propertyName, NumbersListsAndLetters.AutoIncrement, RandomUtil.stringToList(list)));
        return this;
    }

    public GenConfig randomFromList(String propertyName, List objects) {
        this.fieldConfigurations.add(new ListBasedFieldConfig(propertyName, NumbersListsAndLetters.Random, objects));
        return this;
    }

    public GenConfig nextFromStringList(String propertyName, String list) {
        this.fieldConfigurations.add(new ListBasedFieldConfig(propertyName, NumbersListsAndLetters.AutoIncrement, RandomUtil.stringToList(list)));
        return this;
    }

    public GenConfig nextFromList(String propertyName, List objects) {
        this.fieldConfigurations.add(new ListBasedFieldConfig(propertyName, NumbersListsAndLetters.AutoIncrement, objects));
        return this;
    }
    
    public GenConfig dateTimeNow(String propertyName) {
        this.fieldConfigurations.add(new DateAndTimeFieldConfig(propertyName, DateAndTime.Now));
        return this;
    }

    public GenConfig dateTimeNow(String propertyName, DTConfig restrictor) {
        this.fieldConfigurations.add(new DateAndTimeFieldConfig(propertyName, DateAndTime.Now, restrictor));
        return this;
    }

    public GenConfig dateTimeRandom(String propertyName) {
        this.fieldConfigurations.add(new DateAndTimeFieldConfig(propertyName, DateAndTime.Random));
        return this;
    }
    
    public GenConfig dateTimeRandom(String propertyName, Date beginDate, Date endDate) {
        this.fieldConfigurations.add(new DateAndTimeFieldConfig(propertyName, DateAndTime.Random, beginDate, endDate));
        return this;
    }
    
    public GenConfig dateTimeRandom(String propertyName, Date beginDate, Date endDate, DTConfig restrictor) {
        this.fieldConfigurations.add(new DateAndTimeFieldConfig(propertyName, DateAndTime.Random, beginDate, endDate, restrictor));
        return this;
    }
    

}
