import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Update User', function () {
    it('should update a user ', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        let user_patch = {
            last: faker.person.lastName(),
        }

        let patched_user = {
            first: user.first,
            last: user_patch.last,
            username: user.username,
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let res = await axios.put(`${process.env.INF_API_ENDPOINT}main/user/${user.username}`, 
            user_patch
        )

        //assert
        assert.equal(res.status, 200)
        expect(res.data).to.include(patched_user)
    })

    it('should fail to update a user with no fields', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        let user_patch = {
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let res = await axios.put(`${process.env.INF_API_ENDPOINT}main/user/${user.username}`, 
            user_patch,
            { 
                validateStatus: () => true,
            }
        )

        //assert
        assert.equal(res.status, 400)
    })

    it('should fail to update a username', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        let user_patch = {
            last: faker.person.lastName(),
            userName: faker.internet.userName(),
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let res = await axios.put(`${process.env.INF_API_ENDPOINT}main/user/${user.username}`, 
            user_patch,
            { 
                validateStatus: () => true,
            }
        )

        //assert
        assert.equal(res.status, 400)
    })


    it('should fail to update a user that doesn\'t exist', async function () {
        //arrange
        let user_patch = {
            last: faker.person.lastName()
        }

        //act
        let res = await axios.put(`${process.env.INF_API_ENDPOINT}main/user/${faker.internet.userName()}`, 
            user_patch,
            { 
                validateStatus: () => true,
            }
        )

        //assert
        assert.equal(res.status, 404)
    })
});