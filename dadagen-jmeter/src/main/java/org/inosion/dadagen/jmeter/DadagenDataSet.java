package org.inosion.dadagen.jmeter;

import org.apache.jmeter.config.ConfigTestElement;
import org.apache.jmeter.engine.JMeterEngineException;
import org.apache.jmeter.engine.event.LoopIterationEvent;
import org.apache.jmeter.engine.event.LoopIterationListener;
import org.apache.jmeter.engine.util.NoConfigMerge;
import org.apache.jmeter.threads.JMeterContext;
import org.apache.jmeter.threads.JMeterContextService;
import org.apache.jmeter.threads.JMeterVariables;
import org.apache.jorphan.util.JMeterStopThreadException;

import javax.script.ScriptEngine;

import org.inosion.dadagen.MapOfStringsGenerator;
import org.inosion.dadagen.MapOfStringsGenerator$;
import org.inosion.dadagen.api.ScalaScriptEngine$;
import org.inosion.dadagen.generators.DataGenerator;
import scala.collection.immutable.Map;

import java.util.Iterator;
import scala.tools.nsc.interpreter.*;
import scala.tools.nsc.Settings;

/**
 * Created by rbuckland on 12/05/2015.
 */
public class DadagenDataSet extends ConfigTestElement implements LoopIterationListener, NoConfigMerge {

    private static final long serialVersionUID = 027L;

    public static final String DADAGEN_ITER_KEY = "Dadagen.iterator";
    public static final String DADAGEN_CONFIG = "DadagenDataSet.configuration"; // $NON-NLS-1$
    public static final String THREAD_SHARING_SCOPE = "DadagenDataSet.threadSharingScope";
    public static final String RUN_CONTINUOUS = "DadagenDataSet.runContinuous"; // $NON-NLS-1$
    public static final String NUMBER_OF = "DadagenDataSet.numberOf"; // $NON-NLS-1$
    public static final String STOP_THREAD = "DadagenDataSet.stopThread"; // $NON-NLS-1$
    public static final String VARIABLE_PREFIX = "DadagenDataSet.variablePrefix"; // $NON-NLS-1$

    // used for Local Thread
    private Iterator<Map<String,String>> randomIterator;
    private ScriptEngine engine;

    // not using this at the moment
    public enum ThreadModel {
        PER_THREAD_SCOPE("Each thread has it's own data set"),
        PER_GROUP_SCOPE("Generation shared across a thread group"),
        ALL_THREADS_SCOPE("Generation shared across all threads");

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
    public final static int DEFAULT_NUMBER_OF = 10;
    public final static String DEFAULT_DADAGEN_CONFIG = "\n" +
            "  field { \"id\".rownumber }.\n" +
            "  field { \"uuid\".regexgen (\"[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}\") } .\n" +
            "  field { \"number\".number between 10000 and 90000 }.\n" +
            "  field { \"r_str\".regexgen (\"[A-Z][a-zA-Z]{4}[0-9]{4}\") }.\n" +
            "  field { \"gender\".gender }.\n" +
            "  field { \"firstname\".name firstname }.\n" +
            "  field { \"surname_data\".name surname }.\n" +
            "  field { \"surname\".template (\"${surname_data}-${r_str}\") }.\n" +
            "  field { \"fullname\".template (\"${firstname} ${surname}\") }.\n" +
            "  field { \"email_address\".template(\"TEST_${firstname}.${surname}@noemail.test\") }\n";

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

        // currently not looking at the thread group model

        JMeterContext jmeterContext = JMeterContextService.getContext();
        JMeterVariables variables = jmeterContext.getVariables();

        if (! isRunContinuous()) {
            if (loopIterationEvent.getIteration() > getNumberOfLimit() &&
                    isStopThread()) {
                throw new JMeterStopThreadException("Iteration Limit Reached");
            }
        }

        try {
            Map<String,String> randData = getNextRandomData();
            scala.collection.Iterator keys = randData.keysIterator();
            while (keys.hasNext()) {
                String key = (String)keys.next();
                String prefix = getPropertyAsString(VARIABLE_PREFIX,"");
                if (prefix.isEmpty()) {
                    variables.put(key, randData.apply(key));
                } else {
                    variables.put(prefix + key, randData.apply(key));
                }

            }
            variables.put("Dadagen.iteration_id", Integer.toString(loopIterationEvent.getIteration()));
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    private Map<String,String> getNextRandomData() throws Exception {

        if (randomIterator == null) {
            randomIterator = createRandomIterator();
        }
        return randomIterator.next();

    }


    @SuppressWarnings("unchecked")
    private synchronized Iterator<Map<String,String>> createRandomIterator() throws JMeterEngineException {
        if (engine == null) {
            engine = ScalaScriptEngine$.MODULE$.loadEngine();
        }

        Iterator<Map<String,String>> randomMapIterator;

        // we expect that the config in the JMeter GUI is a "field { ... } , field { .. } "
        try {

            scala.collection.immutable.List<DataGenerator<?>> generators =
                    (scala.collection.immutable.List<DataGenerator<?>>) engine.eval(
                            // import the scala implicits
                            "import org.inosion.dadagen.api.scaladsl._\n\n\n" + getDadagenConfig()
                    );
            MapOfStringsGenerator dadagen = MapOfStringsGenerator$.MODULE$.apply(generators, new java.security.SecureRandom());
            randomMapIterator = dadagen.generateJava();
        } catch (Exception e) {
            throw new JMeterEngineException("There was a problem reading the Dadagen Config",e);
        }

        return randomMapIterator;

    }



}
