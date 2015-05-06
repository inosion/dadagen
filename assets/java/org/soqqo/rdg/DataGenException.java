package org.soqqo.rdg;

public class DataGenException extends RuntimeException {

    private static final long serialVersionUID = 4135904740260477743L;

    public DataGenException() {
        super();
    }

    public DataGenException(String message, Throwable cause) {
        super(message, cause);
    }

    public DataGenException(String message) {
        super(message);
    }

    public DataGenException(Throwable cause) {
        super(cause);
    }

}
