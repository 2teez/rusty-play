class NegativeNotEven extends Error {
  constructor(msg: string) {
    super(msg);
  }
}

type NegationError = boolean | NegativeNotEven

const isEven = (num: number): NegationError => {
    if (num < 0) {
        throw new NegativeNotEven("Negative not even.");
    }
    return 0 === num % 2;
};

try {
  isEven(-4);
} catch(nne) {
    console.log(nne);
}
