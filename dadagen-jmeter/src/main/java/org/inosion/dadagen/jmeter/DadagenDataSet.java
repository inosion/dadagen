package org.inosion.dadagen.jmeter;

import org.apache.jmeter.config.ConfigTestElement;
import org.apache.jmeter.engine.JMeterEngineException;
import org.apache.jmeter.engine.event.LoopIterationEvent;
import org.apache.jmeter.engine.event.LoopIterationListener;
import org.apache.jmeter.engine.util.NoConfigMerge;
import org.apache.jmeter.testbeans.TestBean;
import org.apache.jmeter.testelement.property.StringProperty;
import org.apache.jmeter.threads.JMeterContextService;
import org.apache.jmeter.threads.JMeterVariables;

import javax.script.ScriptEngine;
import javax.script.ScriptEngineManager;
import org.inosion.dadagen.MapOfStringsGenerator;
import org.inosion.dadagen.MapOfStringsGenerator$;
import org.inosion.dadagen.randomtypes.DataGenerator;
import scala.collection.immutable.Map;

import java.util.Iterator;
import scala.tools.nsc.interpreter.*;

/**
 * Created by rbuckland on 12/05/2015.
 */
public class DadagenDataSet extends ConfigTestElement implements LoopIterationListener, NoConfigMerge {

    private static final long serialVersionUID = 027L;

    public static final String DADAGEN_CONFIG = "DadagenDataSet.configuration"; // $NON-NLS-1$
    public static final String THREAD_SHARING_SCOPE = "DadagenDataSet.threadSharingScope";
    public static final String RUN_CONTINUOUS = "DadagenDataSet.runContinuous"; // $NON-NLS-1$
    public static final String NUMBER_OF = "DadagenDataSet.numberOf"; // $NON-NLS-1$
    public static final String STOP_THREAD = "DadagenDataSet.stopThread"; // $NON-NLS-1$
    public static final String VARIABLE_PREFIX = "DadagenDataSet.variablePrefix"; // $NON-NLS-1$

    private Iterator<Map<String,String>> randomIterator;
    private ScriptEngine engine;

    public enum ThreadModel {
        PER_THREAD_SCOPE("Each thread has it's own data set"), ALL_THREADS_SCOPE("Data shared across threads");

        private final String threadModel;

        private ThreadModel(String value) {
            threadModel = value;
        }

        @Override
        public String toString() {
            return threadModel;
        }
    }

    // default values
    public final static String DEFAULT_THREAD_SCOPE = ThreadModel.ALL_THREADS_SCOPE.threadModel;
    public final static boolean DEFAULT_RUN_CONTINUOUS = false;
    public final static boolean DEFAULT_STOP_THREAD = true;
    public final static int DEFAULT_NUMBER_OF = 100;

    public String getThreadScope() {return getPropertyAsString(THREAD_SHARING_SCOPE, DEFAULT_THREAD_SCOPE); }
    public void setThreadScope(String threadScope) { setProperty(THREAD_SHARING_SCOPE, threadScope); }

    public boolean isRunContinuous() {return getPropertyAsBoolean(RUN_CONTINUOUS, DEFAULT_RUN_CONTINUOUS);}
    public void setRunContinuous(boolean runContinuous) { setProperty(RUN_CONTINUOUS,runContinuous);}

    public int getNumberOfLimit() { return getPropertyAsInt(NUMBER_OF, DEFAULT_NUMBER_OF); }
    public void setNumberOfLimit(int numberOfLimit) { setProperty(NUMBER_OF, numberOfLimit); }

    public boolean isStopThread() {return getPropertyAsBoolean(STOP_THREAD, DEFAULT_STOP_THREAD);}
    public void setStopThread(boolean stopThread) { setProperty(STOP_THREAD,stopThread);}

    public String getVariablePrefix() { return getPropertyAsString(VARIABLE_PREFIX); }
    public void setVariablePrefix(String prefix) { setProperty(VARIABLE_PREFIX, prefix);}

    public String getDadagenConfig() { return getPropertyAsString(DADAGEN_CONFIG); }
    public void setDadagenConfig(String config) { setProperty(DADAGEN_CONFIG, config);}

    @Override
    public void iterationStart(LoopIterationEvent loopIterationEvent) {

        // in here is where we will load the dadagen config, interpret it and generate our "stuff"

        JMeterVariables variables= JMeterContextService.getContext().getVariables();

        try {
            Map<String,String> randData = getNextRandomData();
            scala.collection.Iterator keys = randData.keysIterator();
            while (keys.hasNext()) {
                String key = (String)keys.next();
                variables.put(key, randData.apply(key));
            }
            variables.put("_dadagen_id", new Integer(loopIterationEvent.getIteration()).toString());
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    private Map<String,String> getNextRandomData() throws Exception{

        if (engine == null) {
            engine = new ScriptEngineManager().getEngineByName("scala");
            if (engine == null) {
                throw new JMeterEngineException("Could not instantiate the Scala Script Engine");
            }

            IMain engineMain = (IMain)engine;
            // we tell scala to be use this class
            // this is the equivalent of
            engineMain.settings().embeddedDefaults(MapOfStringsGenerator.class.getClassLoader());

        }

        if (randomIterator == null) {

            // we expect that the config in the JMeter GUI is a "field { ... } , field { .. } "
            try {

                scala.collection.immutable.List<DataGenerator<?>> generators = (scala.collection.immutable.List<DataGenerator<?>>) engine.eval(getDadagenConfig());
                MapOfStringsGenerator dadagen = MapOfStringsGenerator$.MODULE$.apply(generators, new java.security.SecureRandom());
                randomIterator = dadagen.generateJava();
            } catch (Exception e) {
                throw new JMeterEngineException("There was a problem reading the Dadagen Config",e);
            }
        }

        return randomIterator.next();

//        HashMap<String,String> x = new HashMap<String,String>();
//        x.put(NUMBER_OF,getNumberOfLimit() + "");
//        x.put(THREAD_SHARING_SCOPE, getThreadScope());
//        x.put(VARIABLE_PREFIX,getVariablePrefix());
//        x.put(DADAGEN_CONFIG,getDadagenConfig());
//        return x;
    }


}
