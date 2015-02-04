package org.soqqo.rdg.config;


public class RandomObjectGeneratingFieldConfig extends DataFieldConfig {

    private GenConfig objectGenerationConfig;
    
    private Class theGeneratedClass;

    public RandomObjectGeneratingFieldConfig(String propertyName, GenConfig objectGenerationConfig, Class theGeneratedClass) {
        super(propertyName, DataTypes.SubObject.ObjectGeneration);
        this.setObjectGenerationConfig(objectGenerationConfig);
        this.theGeneratedClass = theGeneratedClass;
    }

    public GenConfig getObjectGenerationConfig() {
        return objectGenerationConfig;
    }

    public void setObjectGenerationConfig(GenConfig objectGenerationConfig) {
        this.objectGenerationConfig = objectGenerationConfig;
    }

    public Class getTheGeneratedClass() {
        return theGeneratedClass;
    }

    public void setTheGeneratedClass(Class theGeneratedClass) {
        this.theGeneratedClass = theGeneratedClass;
    }

}
