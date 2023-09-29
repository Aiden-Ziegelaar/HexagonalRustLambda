import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Create User', function () {
    it('should create a user ', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        //act
        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        //assert
        assert.equal(res.status, 201)
        expect(res.data).to.include(user)
    })

    it('should create a user then fail on shared username', async function () {
        //arrange
        let shared_username = faker.internet.userName()

        let user1 = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: shared_username,
        }

        let user2 = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: shared_username,
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user1)
        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user2, {
            validateStatus: () => true,
        })

        //assert
        assert.equal(res.status, 409)
    })

    it('should create a user then fail on shared email', async function () {
        //arrange
        let shared_email = faker.internet.email().toLocaleLowerCase()

        let user1 = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: shared_email,
            username: faker.internet.userName()
        }

        let user2 = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: shared_email,
            username: faker.internet.userName()
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user1)
        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user2, {
            validateStatus: () => true,
        })

        //assert
        assert.equal(res.status, 409)
    })
});