import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Update User Email', function () {
    it('should update an email', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        let email_patch = {
            email: faker.internet.email().toLowerCase(),
        }

        let patched_user = {
            ...user,
            email: email_patch.email,
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let put_res = await axios.put(`${process.env.INF_API_ENDPOINT}main/user/${user.username}/email`, 
            email_patch
        ).catch(err => {
            console.log(err)
        })

        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/user/${user.username}`)

        //assert
        assert.equal(put_res.status, 200)
        assert.equal(res.status, 200)
        expect(res.data).to.include(patched_user)
    })

    it('should allow creation of a user with an email that was previously taken after update', async function () {
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
            email: user.email,
            username: faker.internet.userName(),
        }

        let email_patch = {
            email: faker.internet.email().toLowerCase(),
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let put_res = await axios.put(`${process.env.INF_API_ENDPOINT}main/user/${user.username}/email`, 
            email_patch,
        )

        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user_after_update)

        //assert
        assert.equal(put_res.status, 200)
        expect(res.data).to.include(user_after_update)
        assert.equal(res.status, 201)
    })

    it('should not allow creation of a user with an email that was updated to', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        let email_patch = {
            email: faker.internet.email().toLowerCase(),
        }

        let user_after_update = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: email_patch.email,
            username: faker.internet.userName(),
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        await axios.put(`${process.env.INF_API_ENDPOINT}main/user/${user.username}/email`, 
            email_patch
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