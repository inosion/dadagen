package org.inosion.dadagen.randomtypes

import org.inosion.dadagen.Context

/**
 * @author rbuckland
 */
case class TemplateGenerator( name:String,
                              template:String
                              ) extends DataGenerator[String] {

  // our regexp for finding our variables within a template
  val varRegex = """(\$\{(\w+)\})""".r

  val description = "Template Variable. Use ${..} names for other fields to build up a custom value"

  override val dependencies = (for { m <- varRegex.findAllMatchIn(template) } yield m.group(2)).toList

  override def internalGenerate(context: Context): String = {
    varRegex.replaceAllIn(template,a => s"${context.dataFieldState(a.group(2))}")
  }
}
