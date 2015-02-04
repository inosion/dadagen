package org.soqqo.rdg.config;

public class DataFieldConfig {

    private int constructorPosition;

    private String propertyName;

    public DataFieldConfig(String propertyName, Enum generationType) {
        this.propertyName = propertyName;
        this.generationType = generationType;
    }  

    /**
     * One of the DataTypes.* enums
     */
    private Enum generationType;

    public String getPropertyName() {
        return propertyName;
    }

    public void setPropertyName(String propertyName) {
        this.propertyName = propertyName;
    }

    public int getConstructorPosition() {
        return constructorPosition;
    }

    public void setConstructorPosition(int constructorPosition) {
        this.constructorPosition = constructorPosition;
    }

    public Enum getGenerationType() {
        return generationType;
    }

    public void setGenerationType(Enum generationType) {
        this.generationType = generationType;
    }
}
