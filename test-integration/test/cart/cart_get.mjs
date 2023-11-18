import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Get Cart', function () {
    it('should add a product to a users cart then retrieve the cart', async function () {
        //arrange
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        //arrange
        let product = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        let user_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)
        let product_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)

        let product_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/product/${product_res.data.id}`)
        let user_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/user/${user_res.data.username}`)

        let cart_item = {
            product_id: product_get.data.id,
            quantity: faker.number.int({
                min: 1, 
                max: 10
            })
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_get.data.username}/item`, cart_item)

        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_get.data.username}`)

        //assert
        assert.equal(res.status, 200)
        expect(res.data[0]).to.include(cart_item)
    })

    it('should add multiple products to a users cart then retrieve the cart', async function () {
        //arrange
        this.timeout(10000);
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        //arrange
        let product = Array(5).fill().map(() => {
            return {
                product_name: faker.commerce.productName(),
                description: faker.commerce.productDescription(),
                price_cents: Number(faker.commerce.price({
                    dec: 0
                }))
            }
        })

        let user_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)
        let product_res = await Promise.all(product.map((product) => axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)))

        let cart_items = Array(5).fill().map((_value, index) => {
                return {
                    product_id: product_res[index].data.id,
                    quantity: faker.number.int({
                        min: 1, 
                        max: 10
                    })
                }
            })

        //act
        for(let cart_item of cart_items) {
            await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_res.data.username}/item`, cart_item)
            await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_res.data.username}`) // wait for strong consistency
        }

        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_res.data.username}`)

        //assert
        assert.equal(res.status, 200);
        assert.equal(res.data.length, 5);
    })

    it('should return an empty array for no cart', async function () {
        //arrange
        let user_id = faker.internet.userName();

        //act
        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}`)

        //assert
        assert.equal(res.status, 200);
        assert.equal(res.data.length, 0);
    })
});