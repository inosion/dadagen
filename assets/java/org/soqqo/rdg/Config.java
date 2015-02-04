package org.soqqo.rdg;

import java.util.HashMap;
import java.util.Set;

public class Config {
    private HashMap<Enum, PropertyConfig> configuration = new HashMap();

    public Set<Enum> propertyTypes() {
        return this.configuration.keySet();
    }

    public PropertyConfig configuration(Enum propertyType) {
        return (PropertyConfig) this.configuration.get(propertyType);
    }

    public Config name(DataTypes.Name nameType, String propertyName, String nameParameters) {
        this.configuration.put(nameType, new PropertyConfig(propertyName, nameParameters));
        return this;
    }

    public Config address(DataTypes.Address addressType, String propertyName, String addressParameters) {
        this.configuration.put(addressType, new PropertyConfig(propertyName, addressParameters));
        return this;
    }

    class PropertyConfig {
        String propertyName;
        String propertyParameters;

        public PropertyConfig(String n, String p) {
            this.propertyName = n;
            this.propertyParameters = p;
        }
    }
}