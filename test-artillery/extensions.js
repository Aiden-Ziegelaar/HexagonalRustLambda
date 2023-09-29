const { faker } = require('@faker-js/faker');

function setJSONBody(requestParams, context, ee, next) {
    requestParams.body = JSON.stringify({
        "first": faker.person.firstName(),
        "last": faker.person.lastName(),
        "email": faker.internet.email(),
        "username": faker.internet.userName(),
    });
    return next(); // MUST be called for the scenario to continue
}

module.exports = {
    setJSONBody,
};