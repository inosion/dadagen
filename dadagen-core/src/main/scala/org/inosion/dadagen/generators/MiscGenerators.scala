package org.inosion.dadagen.generators

object MiscGenerators {
  def uuidGenerator = RegexGenerator("uuid","[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
}