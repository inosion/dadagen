package org.inosion.dadagen.auto

import org.inosion.dadagen.Dadagenerator
trait BasicFormats {

  implicit object IntDadagenFormat extends BaseGenerator[Int] {

    override def build(obj: Int): Dadagenerator[Int] = ???

  }

}
