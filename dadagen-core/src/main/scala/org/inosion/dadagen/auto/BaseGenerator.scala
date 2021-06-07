package org.inosion.dadagen.auto

import org.inosion.dadagen.Dadagenerator

trait BaseGenerator[T] {
  def build(obj: T): Dadagenerator[T]
}
