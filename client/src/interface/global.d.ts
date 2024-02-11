declare type Callback<T = unknown, P = Error | null> = (error?: P, data?: T) => void
