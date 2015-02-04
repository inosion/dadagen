package org.soqqo.rdg;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

import org.apache.commons.beanutils.PropertyUtils;
import org.soqqo.rdg.config.DataFieldConfig;
import org.soqqo.rdg.config.DataTypes;
import org.soqqo.rdg.config.DataTypes.DateAndTime;
import org.soqqo.rdg.config.DateAndTimeFieldConfig;
import org.soqqo.rdg.config.ListBasedFieldConfig;
import org.soqqo.rdg.config.GenConfig;
import org.soqqo.rdg.config.RandomObjectGeneratingFieldConfig;

public class RandomDataGenerator {

    public <T> T generate(GenConfig config, Class<T> clazz) {
        Object generatedObject = null;
        try {
            generatedObject = clazz.newInstance();
        } catch (InstantiationException e) {
            e.printStackTrace();
        } catch (IllegalAccessException e) {
            e.printStackTrace();
        }

        for (DataFieldConfig fieldConfig : config.getFieldConfigurations()) {

            if (fieldConfig.getGenerationType().getDeclaringClass().equals(DataTypes.Name.class)) {
                processNameProperty((DataTypes.Name) fieldConfig.getGenerationType(), (T) generatedObject, config, fieldConfig);
            } else

            if (fieldConfig.getGenerationType().getDeclaringClass().equals(DataTypes.NumbersListsAndLetters.class)) {
                processNumbersListsAndLetters((DataTypes.NumbersListsAndLetters) fieldConfig.getGenerationType(), generatedObject, config, (ListBasedFieldConfig) fieldConfig);
            } else

            if (fieldConfig.getGenerationType().getDeclaringClass().equals(DataTypes.SubObject.class)) {
                processRandomObjectGenerating((DataTypes.SubObject) fieldConfig.getGenerationType(), generatedObject, config, (RandomObjectGeneratingFieldConfig) fieldConfig);
            } else

            if (fieldConfig.getGenerationType().getDeclaringClass().equals(DataTypes.DateAndTime.class)) {
                processDateAndTimeGen((DataTypes.DateAndTime) fieldConfig.getGenerationType(), generatedObject, config, (DateAndTimeFieldConfig) fieldConfig);
            }

            /*
             * else if (Arrays.binarySearch(DataTypes.Address.values(), propertyType.toString()) > 0) {
             * processAddressEntry(ff, config, (DataTypes.Address) propertyType, propertyName, propertyParameters);
             * } else if (Arrays.binarySearch(DataTypes.Contact.values(), propertyType.toString()) > 0) {
             * processContactEntry(ff, config, (DataTypes.Contact) propertyType, propertyName, propertyParameters);
             * } else if (Arrays.binarySearch(DataTypes.HumanAttributes.values(), propertyType.toString()) > 0) {
             * processHumanAttributesEntry(ff, config, (DataTypes.HumanAttributes) propertyType, propertyName, propertyParameters);
             * } else if (Arrays.binarySearch(DataTypes.NumbersAndLetters.values(), propertyType.toString()) > 0) {
             * processNumbersAndLettersEntry(ff, config, (DataTypes.NumbersAndLetters) propertyType, propertyName, propertyParameters);
             * } else if (Arrays.binarySearch(DataTypes.DateAndTime.values(), propertyType.toString()) > 0) {
             * processDateAndTimeEntry(ff, config, (DataTypes.DateAndTime) propertyType, propertyName, propertyParameters);
             * }
             */
        }
        return (T) generatedObject;

    }

    private <T> void processDateAndTimeGen(DateAndTime generationType,  T targetObject, GenConfig config, DateAndTimeFieldConfig fieldConfig) {
        try {
            PropertyUtils.setSimpleProperty(targetObject, fieldConfig.getPropertyName(), fieldConfig.getNextValue());
        } catch (Exception e) {
            throw new DataGenException("Failed to set date/time property [" + fieldConfig.getPropertyName() + "]", e);
        }
    }

    public <T> Set<T> generateSet(int numberToGenerate, GenConfig config, Class<T> clazz) {
        HashSet<T> returnSet = new HashSet<T>();
        for (int i = 0; i < numberToGenerate; i++) {
            returnSet.add((T) generate(config, clazz));
        }
        return returnSet;
    }

    public <T> List<T> generateList(int numberToGenerate, GenConfig config, Class<T> clazz) {
        ArrayList<T> returnList = new ArrayList<T>();
        for (int i = 0; i < numberToGenerate; i++) {
            returnList.add((T) generate(config, clazz));
        }
        return returnList;
    }

    /**
     * Creates a name from a random list of names
     * 
     * @param nameType
     * @param targetObject
     * @param config
     * @param fieldConfig
     */
    private <T> void processNameProperty(DataTypes.Name nameType, T targetObject, GenConfig config, DataFieldConfig fieldConfig) {

        try {
            PropertyUtils.setSimpleProperty(targetObject, fieldConfig.getPropertyName(), RandomUtil.randomName(nameType));
        } catch (Exception e) {
            throw new DataGenException("Failed to set the property [" + fieldConfig.getPropertyName() + "]", e);
        }
    }

    private <T> void processRandomObjectGenerating(DataTypes.SubObject propertyType, T targetObject, GenConfig config, RandomObjectGeneratingFieldConfig fieldConfig) {

        switch (propertyType) {
        case ObjectGeneration: {
            try {
                PropertyUtils.setSimpleProperty(targetObject, fieldConfig.getPropertyName(), generate(fieldConfig.getObjectGenerationConfig(), fieldConfig.getTheGeneratedClass()));
            } catch (Exception e) {
                throw new DataGenException("Failed to set the property [" + fieldConfig.getPropertyName() + "]", e);
            }
            break;
        }
        }

    }

    private <T> void processNumbersListsAndLetters(DataTypes.NumbersListsAndLetters numbersListsAndLettersType, T targetObject, GenConfig config, ListBasedFieldConfig fieldConfig) {

        switch (numbersListsAndLettersType) {

        case Random: {
            try {
                PropertyUtils.setSimpleProperty(targetObject, fieldConfig.getPropertyName(), RandomUtil.randomFromList(fieldConfig.getValueSet()));
            } catch (Exception e) {
                throw new DataGenException("Failed to set the property [" + fieldConfig.getPropertyName() + "] as a random value from list", e);
            }
            break;
        }

        case AutoIncrement: {
            try {
                PropertyUtils.setSimpleProperty(targetObject, fieldConfig.getPropertyName(), fieldConfig.getNextValue());
            } catch (Exception e) {
                throw new DataGenException("Failed to set the property [" + fieldConfig.getPropertyName() + "] as a random value from list", e);
            }
            break;
        }

        }
    }

    private <T> void processContactEntry(T targetObject, GenConfig config, DataTypes.Contact contactType, String propertyName, String propertyParameters) {}

    private <T> void processHumanAttributesEntry(T targetObject, GenConfig config, DataTypes.HumanAttributes humanAttributesType, String propertyName, String propertyParameters) {}

    private <T> void processDateAndTimeEntry(T targetObject, GenConfig config, DataTypes.DateAndTime dateAndTimeType, String propertyName, String propertyParameters) {}

    private <T> void processAddressEntry(T targetObject, GenConfig config, DataTypes.Address addressType, String propertyName, String propertyParameters) {}

}
