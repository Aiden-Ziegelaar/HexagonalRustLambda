import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Delete User', function () {
    it('should delete a user thats just been created', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let res = await axios.delete(`${process.env.INF_API_ENDPOINT}main/user`,
            { params: { email: user.email } }
        )

        //assert
        assert.equal(res.status, 200)
    })

    it('should fail to delete a user that doesn\'t exist', async function () {
        //arrange

        //act
        let res = await axios.delete(`${process.env.INF_API_ENDPOINT}main/user`,
            { 
                params: { email: faker.internet.email() },
                validateStatus: () => true,
            }
        )

        //assert
        assert.equal(res.status, 404)
    })
});