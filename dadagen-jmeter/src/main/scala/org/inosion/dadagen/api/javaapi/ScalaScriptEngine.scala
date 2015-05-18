package org.inosion.dadagen.api.javaapi

import javax.script.{ScriptEngine, ScriptEngineManager}

import org.inosion.dadagen.RandomDataGenerator
import scala.tools.nsc.interpreter._

object ScalaScriptEngine {

  def loadEngine(): ScriptEngine = {
    val engine = new ScriptEngineManager().getEngineByName("scala").asInstanceOf[IMain]

    val cl: ClassLoader = classOf[RandomDataGenerator[_]].getClassLoader
    engine.settings.usejavacp.value = true
    engine
//
//    // we tell scala to be use this class
//    // this is the equivalent of
//    //engineMain.settings().embeddedDefaults(MapOfStringsGenerator.class.getClassLoader());
//    val settings: Nothing = engineMain.settings
//    settings.embeddedDefaults(cl)
//    settings.explicitParentLoader_$eq(scala.Option.apply(cl))
  }

}
