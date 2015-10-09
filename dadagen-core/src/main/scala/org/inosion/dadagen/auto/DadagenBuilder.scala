package org.inosion.dadagen.auto

import org.inosion.dadagen.Dadagenerator

trait DadagenBuilder[T] {
  def makeGenerator(): Dadagenerator[T]
}
