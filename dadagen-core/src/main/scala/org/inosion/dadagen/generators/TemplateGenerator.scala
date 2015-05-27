package org.inosion.dadagen.generators

import java.util.Random

import org.inosion.dadagen.Context

/**
 * @author rbuckland
 */
case class TemplateGenerator( name:String,
                              template:String
                              )(implicit rand: Random) extends DataGenerator[String] {

  // our regexp for finding our variables within a template
  val varRegex = """(\$\{(\w+)\})""".r

  val description = "Template Variable. Use ${..} names for other fields to build up a custom value"

  override val dependencies = (for { m <- varRegex.findAllMatchIn(template) } yield m.group(2)).toList

  override def internalGenerate(context: Context)(implicit rand: Random): String = {
    varRegex.replaceAllIn(template,a => s"${context.dataFieldState(a.group(2))}")
  }
}