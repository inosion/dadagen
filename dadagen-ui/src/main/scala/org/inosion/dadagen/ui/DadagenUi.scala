package org.inosion.dadagen.ui

import org.fife.ui.rsyntaxtextarea.{SyntaxConstants, RSyntaxTextArea, RSyntaxTextAreaUI}
import org.fife.ui.rtextarea.RTextScrollPane
import org.inosion.dadagen.generators.RandomUtil
import org.inosion.dadagen.lists.{ListConfigSupport, ListManager}

import scala.concurrent.Future
import scalafx.application.JFXApp
import scalafx.application.JFXApp.PrimaryStage
import scalafx.embed.swing.SwingNode
import scalafx.geometry.{Pos, Insets}
import scalafx.scene.Scene
import scalafx.scene.control._
import scalafx.scene.effect._
import scalafx.scene.input.MouseEvent
import scalafx.scene.layout._
import scalafx.scene.paint.Color._
import scalafx.scene.paint.{LinearGradient, Stops}
import scalafx.stage.FileChooser
import scalafx.Includes._

object DadagenUi extends JFXApp {

  implicit val rand = new java.security.SecureRandom()

  val PotatoList = "miscellaneous.potato-varieties"
  val ColourList = "miscellaneous.colours"

  ListConfigSupport.importConfigListData(PotatoList)
  ListConfigSupport.importConfigListData(ColourList)


  val Padding = 10
  val DefaultWidth = 830
  val DefaultHeight = 800
  val DefaultRows = 35
  val DefaultCols = 102
  val SampleConfig =
    """
      |  // this import allows us to use the Case Class directly.
      |  //   field { DoubleGenerator("initial_investment",1000,80000,2) }.
      |
      |  import org.inosion.dadagen.generators._
      |
      |  field { "id".rownumber }.
      |  field { "r_uuid".regexgen ("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}") } .
      |  field { "r_rand1".number between 10000 and 90000 }.
      |  field { "r_str".regexgen ("[A-Z][a-zA-Z]{4}[0-9]{4}") }.
      |  field { "payload_id".template ("PERFT_${id}_${r_uuid}") }.
      |  field { "gender".gender }.
      |  field { "firstname".name firstname }.
      |  field { "surname_data".name surname }.
      |  field { "surname".template ("${surname_data}-${r_str}") }.
      |  field { "fullname".template ("${firstname} ${surname}") }.
      |  field { "dob".regexgen ("19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])") }.
      |  field { "email_address".template("TEST_${firstname}.${surname}@noemail.test") }.
      |  field { "nino".regexgen("(A|B|C|E|G|H|J|K|L|M|N|O|P|R|S|T|W|X|Y|Z){2}[0-9]{6}A") }.
      |  field { "street_number".number between 1 and 100 }.
      |  field { "street_name".template ("RS Performance Street" ) }.
      |  field { "town".address city }.
      |  field { "postcode".regexgen ("[A-Z][A-Z][0-9] [0-9][A-Z][A-Z]") }.
      |  // Issue #10 - API is not supporting precision right now, but the case class does (the default _is_ 2.. see "upfront_commission") .. but if you want more precision, this is how
      |  field { DoubleGenerator("initial_investment",1000,80000,2) }.
      |  field { "regular_investment_amount".regexgen("(50|100|150|200|250|300|350|400|450|500|550|600|650|700|750|800|850|900|950)") }.
      |  field { "account_number".number between 8800000 and 8899999 }.
      |  field { "sort_code".regexgen("(402205|110124|830608|880011|938424|938343|938130)") }.
      |  field { "mobile_phone_number".regexgen ("07777 [0-9]{3} [0-9]{3}") }.
      |  field { "retirement_age".number between 65 and 75 }.
      |  field { "upfront_commission".number between 100.00 and 150.00 }. // when using floats, the default is precision 2 (that is, this will create eg 110.18 )
      |  field { "commission_percentage".number between 0.01 and 0.05 }
      |
    """.stripMargin

  stage = new PrimaryStage {
    title = "Dadagen"
    width = DefaultWidth
    height = DefaultHeight
    scene = new Scene {
      fill = LightGoldrenrodYellow
      content = mainPane
    }
    minWidth = DefaultWidth
  }

  /*
   * Main panel
   */
  def mainPane = new BorderPane {
    padding = Insets(Padding)
    center = rsyntaxArea
    bottom = bottomPane
  }

  /*
   * Main Editing Panel - Create the rsyntax Swing Node
   */
  lazy val rsyntaxArea = {

    val textArea = new RSyntaxTextArea(DefaultRows, DefaultCols)
    textArea.setSyntaxEditingStyle(SyntaxConstants.SYNTAX_STYLE_SCALA)
    textArea.setCodeFoldingEnabled(true)
    textArea.setText(SampleConfig)

    new SwingNode {
      content = new RTextScrollPane(textArea)
    }

  }

  /*
   * Bottom on window
   */
  lazy val bottomPane = new VBox {
    children = Seq(
      messages,
       new HBox {
       padding = Insets(Padding)
       children = Seq(
          numberLabel
          , numberOf
          , fileTypeRadios
          , fileNameField
          , generateButton
        )
        hgrow = Priority.Always
        alignment = Pos.CenterRight
       }
    )
  }

  lazy val messages = new TextArea {
    prefRowCount = 5
//    disable = true
  }

  lazy val messagesPane = new ScrollPane {
    content = messages
    fitToWidth = true
    fitToHeight = true
  }

  /*
   * File Output Type Selectors
   */
  lazy val fileTypeRadios = {
    val jsonButton = new ToggleButton("JSON")
    val csvButton = new ToggleButton {
      text = "CSV"
      selected = true
    }
    val xmlButton = new ToggleButton("XML")

    val gp = new GridPane {
      hgap = 10
      vgap = 1
      alignment = Pos.Center
    }

    gp.add(jsonButton,0,0)
    gp.add(csvButton,1,0)
    gp.add(xmlButton,2,0)

    gp
  }

  def toggleGroup(buttons:ToggleButton*) = {
    new ToggleGroup {
      toggles = buttons
    }
  }

  lazy val numberLabel = new Label("Number of rows / samples")

  lazy val numberOf = new TextField{
    prefColumnCount = 15
    text = "100"
  }

  def updateMessage(msg:String) = if (messages.getText.length > 0 )
                                      messages.setText(messages.getText  + "\n" + msg)
                                  else
                                      messages.setText(msg)

  lazy val generateButton = new Button("Generate Data") {
    onMousePressed = (me: MouseEvent) => {
      import scala.concurrent.ExecutionContext.Implicits.global
      val fxec = JavaFXExecutionContext.javaFxExecutionContext
      Future {
        DadagenErator.generateData(numberOf.getText.toInt, CSVFile, fileNameField.getText, rsyntaxArea.content.asInstanceOf[RTextScrollPane].getTextArea.getText)
      }.map( result => {
        result match {
          case Left(e) => updateMessage("\u25B6 " + e.getLocalizedMessage)
          case Right(e) => updateMessage("\u26AA " + e)
        }
      })(fxec)
    }
  }

  lazy val fileNameField = new TextField {
    text = "/tmp/" + ListManager.getRandomValue("miscellaneous.potato-varieties") +
                   "-" + ListManager.getRandomValue("miscellaneous.colours") +
                   "-" + RandomUtil.randomIntRange(1000,9999) + ".csv"
  }

}