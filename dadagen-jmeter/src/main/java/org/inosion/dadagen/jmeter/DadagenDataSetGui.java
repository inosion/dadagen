package org.inosion.dadagen.jmeter;

import org.apache.jmeter.config.gui.AbstractConfigGui;
import org.apache.jmeter.gui.util.HorizontalPanel;
import org.apache.jmeter.gui.util.JSyntaxTextArea;
import org.apache.jmeter.gui.util.JTextScrollPane;
import org.apache.jmeter.gui.util.VerticalPanel;
import org.apache.jmeter.testelement.TestElement;
import org.apache.jmeter.util.JMeterUtils;
//import org.apache.jorphan.logging.LoggingManager;

import javax.swing.*;
import java.awt.*;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
//import org.apache.log.Logger;

/**
 * Created by rbuckland on 13/05/2015.
 */
public class DadagenDataSetGui extends AbstractConfigGui {

//    private static final Logger log = LoggingManager.getLoggerForClass();

    private static final String ELEMENT_LABEL = "dadagenDataSet";
    private final static String PROJECT_URL = "http://github.com/inosion/dadagen";

    // set of action commands
    public static final String INIFINITE_DATA = "inifiniteData";
    public static final String FINITE_DATA = "finiteData";

    // GUI Stuff
    private JTextField numberOf;
    private JLabel numberOfLabel;
    private JTextField variablePrefix;

    private JComboBox threadScopeModelChooser;
    private JTextField parameters;// parameters to pass to script file (or script)
    private JSyntaxTextArea dadgenConfigField; // script area

    private JRadioButton btnRunContinuous;
    private JRadioButton btnLimited;

    private JRadioButton btnThreadStopYes;
    private JRadioButton btnThreadStopNo;
    private JLabel threadStopLabel;

    /**
     * Ctor - setup some defaults
     */
    public DadagenDataSetGui() {
        super();
        init();
        initFields();
    }


    private void initFields() {
        setName("Dadagen Random Data Generator");
        variablePrefix.setText("");
        numberOf.setText(Integer.toString(DadagenDataSet.DEFAULT_NUMBER_OF));
        dadgenConfigField.setText(DadagenDataSet.DEFAULT_DADAGEN_CONFIG);
        setStopThread(true);
        setSelectedRunContinuous(false);
    }

    public String getLabelResource()
    {
        return getClass().getName();
    }

    @Override
    public String getStaticLabel()
    {
        return "Dadagen Random Data Generator";
    }

    /**
     * Sets up the GUI fields, from the TestElement bean
     * @param element
     */
    @Override
    public void configure(TestElement element) {
        super.configure(element);
        if (element instanceof DadagenDataSet) {
            DadagenDataSet dadagen = (DadagenDataSet)element;

            // threadScopeModelChooser.setSelectedItem(dadagen.getThreadScope());
            setSelectedRunContinuous(dadagen.isRunContinuous());
            numberOf.setText(Integer.toString(dadagen.getNumberOfLimit()));
            setStopThread(dadagen.isStopThread());
            variablePrefix.setText(dadagen.getVariablePrefix());
            dadgenConfigField.setText(dadagen.getDadagenConfig());
        }
    }

    private void setSelectedRunContinuous(boolean isRunContinuous) {
        btnRunContinuous.setSelected(isRunContinuous);
        btnLimited.setSelected(!isRunContinuous);
        setFiniteRunningPropertiesEnabled(!isRunContinuous);
    }

    private void setFiniteRunningPropertiesEnabled(boolean state) {
        numberOf.setEnabled(state);
        numberOfLabel.setEnabled(state);
        btnThreadStopYes.setEnabled(state);
        btnThreadStopNo.setEnabled(state);
        threadStopLabel.setEnabled(state);
    }

    private void setStopThread(boolean isStopThread) {
        btnThreadStopYes.setSelected(isStopThread);
        btnThreadStopNo.setSelected(!isStopThread);
    }
    @Override
    public void clearGui() {
        super.clearGui();
        initFields();
    }

    private void init() {

        setLayout(new BorderLayout(0, 5));
        setBorder(makeBorder());

        add(makeTitlePanel(), BorderLayout.NORTH);

        JPanel mainPanel = new JPanel(new GridBagLayout());

        GridBagConstraints labelConstraints = new GridBagConstraints();
        labelConstraints.anchor = GridBagConstraints.FIRST_LINE_END;

        GridBagConstraints editConstraints = new GridBagConstraints();
        editConstraints.anchor = GridBagConstraints.FIRST_LINE_START;
        editConstraints.weightx = 1.0;
        editConstraints.fill = GridBagConstraints.HORIZONTAL;

        /* TODO  - work out JMeters's way of context threadlocal etc
        addToPanel(mainPanel, labelConstraints, 0, 0, new JLabel(" Thread Data Sharing: ", JLabel.RIGHT));
        addToPanel(mainPanel, editConstraints, 1, 0, makethreadScopeModel());
        */

        // this sets up the btnRunContinuous and btnLimited buttons

        btnRunContinuous = new JRadioButton();
        btnRunContinuous.setActionCommand(INIFINITE_DATA);
        btnRunContinuous.addActionListener(continuousSupply);

        btnLimited = new JRadioButton();
        btnLimited.setActionCommand(FINITE_DATA);
        btnLimited.addActionListener(continuousSupply);

        addToPanel(mainPanel, labelConstraints, 0, 1, new JLabel(" Continuous: ", JLabel.RIGHT));
        addToPanel(mainPanel, editConstraints, 1, 1, btnRunContinuous);

        addToPanel(mainPanel, labelConstraints, 0, 2, new JLabel(" Finite: ", JLabel.RIGHT));
        addToPanel(mainPanel, editConstraints, 1, 2, btnLimited);

        /* vvv these options are only for finite running */
        addToPanel(mainPanel, labelConstraints, 0, 3, numberOfLabel = new JLabel(" Number of rows: ", JLabel.RIGHT));
        addToPanel(mainPanel, editConstraints, 1, 3, numberOf = new JTextField(20));

        addToPanel(mainPanel, labelConstraints, 0, 4, threadStopLabel = new JLabel(" Stop thread when exhausted rows: ", JLabel.RIGHT));

        HorizontalPanel yesNoThread = new HorizontalPanel();
        yesNoThread.add(btnThreadStopYes = new JRadioButton("Stop Thread"));
        yesNoThread.add(btnThreadStopNo = new JRadioButton("Don't Stop"));

        ButtonGroup group = new ButtonGroup();
        group.add(btnLimited);
        group.add(btnRunContinuous);

        ButtonGroup threadGroup = new ButtonGroup();
        threadGroup.add(btnThreadStopYes);
        threadGroup.add(btnThreadStopNo);

        /* ^^^ these options are only for finite running */

        addToPanel(mainPanel, editConstraints, 1, 4, yesNoThread);

        addToPanel(mainPanel, labelConstraints, 0, 5, new JLabel(" Variable prefix: ", JLabel.RIGHT));
        addToPanel(mainPanel, editConstraints, 1, 5, variablePrefix = new JTextField(20));


        JPanel dadgenConfigPanel = new JPanel(new BorderLayout(0, 5));

        JPanel editorPanel = makeDadagenConfigPanel();
        dadgenConfigPanel.add(editorPanel);

        // Don't let the input field shrink too much
        dadgenConfigPanel.add(Box.createVerticalStrut(editorPanel.getPreferredSize().height), BorderLayout.WEST);


        JPanel container = new JPanel(new BorderLayout());

        container.add(mainPanel, BorderLayout.NORTH); // all the configuration options
        container.add(dadgenConfigPanel, BorderLayout.CENTER); // the editor

        add(container, BorderLayout.CENTER);

    }


    /**
     * Create a new Test Element Bean
     * @return
     */
    @Override
    public TestElement createTestElement() {
        DadagenDataSet dadagen = new DadagenDataSet();
        modifyTestElement(dadagen);
        dadagen.setComment("See " + PROJECT_URL + " for doc and examples");
        return dadagen;
    }



    private ActionListener continuousSupply = new ActionListener() {
        public void actionPerformed(ActionEvent e) {
            if (e.getActionCommand().equals(INIFINITE_DATA)) {
                setSelectedRunContinuous(true);
            } else {
                setSelectedRunContinuous(false);
            }
        }
    };

    private JComboBox makethreadScopeModel() {

        DefaultComboBoxModel threadScopeModel = new DefaultComboBoxModel();
        threadScopeModel.addElement(DadagenDataSet.ThreadModel.PER_THREAD_SCOPE);
        threadScopeModel.addElement(DadagenDataSet.ThreadModel.ALL_THREADS_SCOPE);
        threadScopeModelChooser = new JComboBox(threadScopeModel);

        return threadScopeModelChooser;
    }


    private JPanel makeDadagenConfigPanel() {
        dadgenConfigField = new JSyntaxTextArea(20,20);

        JLabel label = new JLabel("Dadagen Configuration (see " + PROJECT_URL + ")"); //$NON-NLS-1$
        label.setLabelFor(dadgenConfigField);

        JPanel panel = new JPanel(new BorderLayout());
        panel.add(label, BorderLayout.NORTH);
        panel.add(new JTextScrollPane(dadgenConfigField), BorderLayout.CENTER);

        return panel;
    }


    /**
     * Modifies a given TestElement to mirror the data in the gui components.
     *
     * @see org.apache.jmeter.gui.JMeterGUIComponent#modifyTestElement(TestElement)
     */
    @Override
    public void modifyTestElement(TestElement te) {
        te.clear();

        DadagenDataSet dadagen = (DadagenDataSet)te;

        dadagen.setProperty(TestElement.GUI_CLASS, DadagenDataSetGui.class.getCanonicalName());
        dadagen.setName(getName());
        dadagen.setProperty(DadagenDataSet.NUMBER_OF, numberOf.getText());

        // thread scope
        /* not using right now
        dadagen.setProperty(DadagenDataSet.THREAD_SHARING_SCOPE,
                ((DadagenDataSet.ThreadModel) threadScopeModelChooser.getSelectedItem()).toString()
        );
        */

        dadagen.setProperty(DadagenDataSet.RUN_CONTINUOUS,btnRunContinuous.isSelected());
        dadagen.setProperty(DadagenDataSet.STOP_THREAD, btnThreadStopYes.isSelected());
        dadagen.setProperty(DadagenDataSet.VARIABLE_PREFIX, variablePrefix.getText());

        // we set the config directly because we don't want JMeter to interpret the value before hand.
        dadagen.setDadagenConfig(dadgenConfigField.getText());

    }

    private void addToPanel(JPanel panel, GridBagConstraints constraints, int col, int row, JComponent component) {
        constraints.gridx = col;
        constraints.gridy = row;
        panel.add(component, constraints);
    }

}
