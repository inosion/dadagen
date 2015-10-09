package org.inosion.dadagen.auto

import org.inosion.dadagen.generators.TypedDataGenerator$

trait BasicFormats {

  implicit object IntDadagenFormat extends DadagenFormat[Int] {

    override def build(obj: Int): IteratingRandomGenerator[Int] = ???

  }

}
