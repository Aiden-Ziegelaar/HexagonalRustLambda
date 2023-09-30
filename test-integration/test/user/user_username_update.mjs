import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Update User', function () {
    it('should update a username', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        let username_patch = {
            email: user.email,
            username: faker.internet.userName(),
        }

        let patched_user = {
            ...user,
            username: username_patch.username,
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        await axios.put(`${process.env.INF_API_ENDPOINT}main/user/username`, 
            username_patch,
            {
                validateStatus: () => true,
            }
        )

        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/user`,
            { params: { email: user.email } }
        )

        //assert
        expect(res.data).to.include(patched_user)
        assert.equal(res.status, 200)
    })
});