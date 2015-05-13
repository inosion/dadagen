package org.inosion.dadagen.jmeter;
import java.beans.PropertyDescriptor;

import org.apache.jmeter.testbeans.BeanInfoSupport;
/**
 * Created by rbuckland on 12/05/2015.
 */
public class DadagenDataSetBeanInfo extends BeanInfoSupport {

    // These group names must have .displayName properties
    private static final String VOLUME_OPTIONS_GROUP = "volumeOptions"; // $NON-NLS-1$
    private static final String CONFIG_GROUP = "dadgenConfig"; // $NON-NLS-1$

    // These variable names must have .displayName properties and agree with the getXXX()/setXXX() methods
    private static final String RUN_CONTINUOUS = "runContinuous"; // $NON-NLS-1$
    private static final String NUMBER_OF = "numberOf"; // $NON-NLS-1$
    private static final String STOP_THREAD = "stopThread"; // $NON-NLS-1$
    private static final String PER_THREAD = "perThread"; // $NON-NLS-1$

    private static final String VARIABLE_PREFIX = "variablePrefix"; // $NON-NLS-1$
    private static final String DADAGEN_CONFIG = "dadagenConfiguration"; // $NON-NLS-1$


    public DadagenDataSetBeanInfo() {
        super(DadagenDataSet.class);

        PropertyDescriptor p;

        createPropertyGroup(VOLUME_OPTIONS_GROUP, new String[] { RUN_CONTINUOUS, NUMBER_OF, STOP_THREAD, PER_THREAD});

        p = property(RUN_CONTINUOUS);
        p.setValue(NOT_UNDEFINED, Boolean.TRUE);
        p.setValue(NOT_EXPRESSION, Boolean.TRUE);
        p.setValue(NOT_OTHER, Boolean.TRUE);
        p.setValue(DEFAULT, Boolean.TRUE); // $NON-NLS-1$

        p = property(NUMBER_OF);
        p.setValue(NOT_UNDEFINED, Boolean.TRUE);
        p.setValue(DEFAULT, 0); // $NON-NLS-1$

        p = property(STOP_THREAD);
        p.setValue(NOT_UNDEFINED, Boolean.TRUE);
        p.setValue(NOT_EXPRESSION, Boolean.TRUE);
        p.setValue(NOT_OTHER, Boolean.TRUE);
        p.setValue(DEFAULT, Boolean.FALSE); // $NON-NLS-1$

        p = property(PER_THREAD);
        p.setValue(NOT_UNDEFINED, Boolean.TRUE);
        p.setValue(NOT_EXPRESSION, Boolean.TRUE);
        p.setValue(NOT_OTHER, Boolean.TRUE);
        p.setValue(DEFAULT, Boolean.FALSE);


        createPropertyGroup(CONFIG_GROUP,
                new String[]{VARIABLE_PREFIX, DADAGEN_CONFIG});

        p = property(VARIABLE_PREFIX);
        p.setValue(DEFAULT, "dg_"); // $NON-NLS-1$

        p = property(DADAGEN_CONFIG);
        p.setValue(NOT_UNDEFINED, Boolean.TRUE);
        p.setValue(NOT_EXPRESSION, Boolean.TRUE);
        //p.setValue(NOT_OTHER, Boolean.TRUE);
        p.setValue(DEFAULT, ""); // $NON-NLS-1$

    }

}
