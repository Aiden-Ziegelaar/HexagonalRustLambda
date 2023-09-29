import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Get User', function () {
    it('should get a user thats just been created', async function () {
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/user`,
            { params: { email: user.email } }
        )

        assert.equal(res.status, 200)
        expect(res.data).to.include(user)
    })

    it('should fail to get a user that doesn\'t exist', async function () {
        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/user`,
            { 
                params: { email: faker.internet.email() },
                validateStatus: () => true,
            }
        )

        assert.equal(res.status, 404)
    })
});