import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Update User Username', function () {
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

    it('should allow creation of a user with a username that was previously taken after update', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        let user_after_update = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: user.username,
        }

        let username_patch = {
            email: user.email,
            username: faker.internet.userName(),
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        await axios.put(`${process.env.INF_API_ENDPOINT}main/user/username`, 
            username_patch,
            {
                validateStatus: () => true,
            }
        )

        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user_after_update)

        //assert
        expect(res.data).to.include(user_after_update)
        assert.equal(res.status, 201)
    })

    it('should not allow creation of a user with a username that was updated to', async function () {
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

        let user_after_update = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
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

        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user_after_update,
            {
                validateStatus: () => true,
            }
        )

        //assert
        assert.equal(res.status, 409)
    })
});