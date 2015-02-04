package org.soqqo.rdg.config;

import java.util.List;

import org.soqqo.rdg.RandomUtil;

public class ListBasedFieldConfig extends DataFieldConfig {

    public ListBasedFieldConfig(String propertyName, DataTypes.NumbersListsAndLetters fieldGenerationType, @SuppressWarnings("rawtypes") List valueSet) {
        super(propertyName, fieldGenerationType);
        this.fieldGenerationType = fieldGenerationType;
        this.valueSet = valueSet;
    }

    private DataTypes.NumbersListsAndLetters fieldGenerationType;

    @SuppressWarnings("rawtypes")
    private List valueSet;

    private int counter = 0;

    @SuppressWarnings("rawtypes")
    public List getValueSet() {
        return valueSet;
    }

    @SuppressWarnings("rawtypes")
    public void setValueSet(List valueSet) {
        this.valueSet = valueSet;
    }

    @SuppressWarnings("unchecked")
    public Object getNextValue() {
        if (fieldGenerationType.equals(DataTypes.NumbersListsAndLetters.AutoIncrement)) {
            // wrap around the list
            if (counter == valueSet.size()) {
                counter = 0;
            }
            return valueSet.get(counter++);
        } else if (fieldGenerationType.equals(DataTypes.NumbersListsAndLetters.Random)) {
            return RandomUtil.randomFromList(valueSet);
        } else {
            return null;
        }

    }

}
