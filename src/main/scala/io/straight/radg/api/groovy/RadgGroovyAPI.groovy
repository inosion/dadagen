package io.straight.radg.api.groovy

// import io.straight.rdg.api.RowCountWrappedInt
import io.straight.radg.randomtypes.DataGenerator

ExpandoMetaClass.enableGlobally();


class DataGeneratorHolder {
    public int rows;

    private List<DataGenerator> generators = new ArrayList<DataGenerator>()

    public DataGeneratorHolder(Integer rows) {
        this.rows = rows;
    }

    def DataGeneratorHolder col(DataGenerator generator) {
        generators.add(generator)
    }
}
Number.metaClass.getProperty = { String method ->
    if (method.equals("rows")) {
        new DataGeneratorHolder(delegate)
    }
}
/*

   // this is what we want the Groovy API too look like for CSV data
   def csvData = randomGenerate 40.rows {
                    col { rownumber },
                    col { name.firstname },
                    col { name.title },
                    col { name.surname },
                    col { address.fulladdress }
                  }

                 }

    // and this is what we want the Groovy Bean API to look like
    def arrayObjects = randomGenerate 40.objects { MyBeanClass.class }

*/