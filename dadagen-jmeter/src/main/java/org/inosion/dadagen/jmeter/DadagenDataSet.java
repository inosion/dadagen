package org.inosion.dadagen.jmeter;

import org.apache.jmeter.config.ConfigTestElement;
import org.apache.jmeter.engine.event.LoopIterationEvent;
import org.apache.jmeter.engine.event.LoopIterationListener;
import org.apache.jmeter.engine.util.NoConfigMerge;
import org.apache.jmeter.testbeans.TestBean;
import org.apache.jmeter.threads.JMeterContextService;
import org.apache.jmeter.threads.JMeterVariables;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

/**
 * Created by rbuckland on 12/05/2015.
 */
public class DadagenDataSet extends ConfigTestElement implements TestBean, LoopIterationListener, NoConfigMerge {

    private static final long serialVersionUID = 027L;

    private transient String variablePrefix;

    private transient boolean runContinuous;

    private transient Integer numberOf;

    private transient boolean stopThread;

    private transient boolean perThread;

    private transient String dadagenConfiguration;

    @Override
    public void iterationStart(LoopIterationEvent loopIterationEvent) {


        // DataGen generator = null;

        /*
            if (getPerThread()){

            } else {

            }
         */

        JMeterVariables variables= JMeterContextService.getContext().getVariables();

        Iterator<Map.Entry<String, String>>  it = getNextRandomData().entrySet().iterator();
        while (it.hasNext()) {
            Map.Entry<String, String> pair = it.next();
            variables.put(pair.getKey(), pair.getValue());
        }
    }


    private Map<String,String> getNextRandomData() {
        HashMap<String,String> x = new HashMap<String,String>();
        x.put("variable","foobar");
        x.put("v009","baz_fff_foobar");
        x.put("someValue","1234567");
        return x;
    }

    public String getVariablePrefix() {
        return variablePrefix;
    }

    public void setVariablePrefix(String variablePrefix) {
        this.variablePrefix = variablePrefix;
    }

    public boolean isRunContinuous() {
        return runContinuous;
    }

    public void setRunContinuous(boolean runContinuous) {
        this.runContinuous = runContinuous;
    }

    public Integer getNumberOf() {
        return numberOf;
    }

    public void setNumberOf(Integer numberOf) {
        this.numberOf = numberOf;
    }

    public boolean isStopThread() {
        return stopThread;
    }

    public void setStopThread(boolean stopThread) {
        this.stopThread = stopThread;
    }

    public boolean isPerThread() {
        return perThread;
    }

    public void setPerThread(boolean perThread) {
        this.perThread = perThread;
    }

    public String getDadagenConfiguration() {
        return dadagenConfiguration;
    }

    public void setDadagenConfiguration(String dadagenConfiguration) {
        this.dadagenConfiguration = dadagenConfiguration;
    }

}
