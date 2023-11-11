import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Get User', function () {
    it('should get a user thats just been created', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/user/${user.username}`)

        //assert
        assert.equal(res.status, 200)
        expect(res.data).to.include(user)
    })

    it('should fail to get a user that doesn\'t exist', async function () {
        //arrange

        //act
        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/user/${faker.internet.userName()}`,
            { 
                validateStatus: () => true,
            }
        )
        
        //assert
        assert.equal(res.status, 404)
    })
});