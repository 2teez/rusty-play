"use strict";
class NegativeNotEven extends Error {
    constructor(msg) {
        super(msg);
    }
}
const isEven = (num) => {
    if (num < 0) {
        throw new NegativeNotEven("Negative not even.");
    }
    return 0 === num % 2;
};
try {
    isEven(-4);
}
catch (nne) {
    console.log(nne);
}
